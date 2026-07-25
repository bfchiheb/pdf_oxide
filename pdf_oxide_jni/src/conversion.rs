//! JNI surface for {@code fyi.oxide.pdf.PdfAConverter} — PDF/A
//! conversion (v0.3.53).
//!
//! Implements `Java_fyi_oxide_pdf_PdfAConverter_nativeConvertToPdfA`
//! which calls through to `pdf_oxide::compliance::convert_to_pdf_a`
//! and marshals the `ConversionResult` with converted bytes back to Java.

use jni::errors::{Error as JniError, ThrowRuntimeExAndDefault};
use jni::jni_sig;
use jni::objects::{JByteArray, JClass, JObject};
use jni::strings::JNIString;
use jni::sys::{jint, jobject};
use jni::EnvUnowned;
use pdf_oxide::compliance::{
    convert_to_pdf_a, ActionType, ConversionResult as RustConversionResult, ErrorCode, PdfALevel,
};
use pdf_oxide::PdfDocument;

use crate::error::throw_pdf;

/// Translate a Java {@code PdfALevel.ordinal()} into the Rust enum.
///
/// Wire format matches `src/ffi.rs:1225` — `0=A1b 1=A1a 2=A2b 3=A2a
/// 4=A2u 5=A3b 6=A3a 7=A3u`. Java's `PdfALevel` is reordered (B before
/// A within each level — see v0.3.55 #547) so `.ordinal()` matches.
fn map_pdfa_ordinal<'local>(env: &mut jni::Env<'local>, ord: jint) -> Result<PdfALevel, JniError> {
    match ord {
        0 => Ok(PdfALevel::A1b),
        1 => Ok(PdfALevel::A1a),
        2 => Ok(PdfALevel::A2b),
        3 => Ok(PdfALevel::A2a),
        4 => Ok(PdfALevel::A2u),
        5 => Ok(PdfALevel::A3b),
        6 => Ok(PdfALevel::A3a),
        7 => Ok(PdfALevel::A3u),
        8..=10 => {
            let cls = JNIString::from("fyi/oxide/pdf/exception/PdfUnsupportedException");
            let msg = JNIString::from("PDF/A-4 levels not yet supported by pdf_oxide");
            env.throw_new(&cls, &msg)?;
            Err(JniError::JavaException)
        },
        _ => {
            let cls = JNIString::from("java/lang/IllegalArgumentException");
            let msg = JNIString::from(format!("unknown PdfALevel ordinal {}", ord));
            env.throw_new(&cls, &msg)?;
            Err(JniError::JavaException)
        },
    }
}

/// Map a Rust `PdfALevel` to the Java enum constant name.
fn pdfa_level_java_name(level: PdfALevel) -> &'static str {
    match level {
        PdfALevel::A1b => "A_1B",
        PdfALevel::A1a => "A_1A",
        PdfALevel::A2b => "A_2B",
        PdfALevel::A2a => "A_2A",
        PdfALevel::A2u => "A_2U",
        PdfALevel::A3b => "A_3B",
        PdfALevel::A3a => "A_3A",
        PdfALevel::A3u => "A_3U",
    }
}

/// Map a Rust `ActionType` to the Java enum constant name.
fn action_type_java_name(action_type: ActionType) -> &'static str {
    match action_type {
        ActionType::AddedXmpMetadata => "ADDED_XMP_METADATA",
        ActionType::AddedPdfaIdentification => "ADDED_PDFA_IDENTIFICATION",
        ActionType::EmbeddedFont => "EMBEDDED_FONT",
        ActionType::AddedOutputIntent => "ADDED_OUTPUT_INTENT",
        ActionType::RemovedJavaScript => "REMOVED_JAVASCRIPT",
        ActionType::RemovedEncryption => "REMOVED_ENCRYPTION",
        ActionType::FlattenedTransparency => "FLATTENED_TRANSPARENCY",
        ActionType::RemovedEmbeddedFiles => "REMOVED_EMBEDDED_FILES",
        ActionType::AddedStructure => "ADDED_STRUCTURE",
        ActionType::FixedAnnotation => "FIXED_ANNOTATION",
        ActionType::AddedLanguage => "ADDED_LANGUAGE",
    }
}

/// Map a Rust `ErrorCode` to an integer for `ConversionAction.fixedErrorCode`.
/// Returns -1 when no error was fixed.
fn error_code_to_int(code: Option<&ErrorCode>) -> jint {
    match code {
        None => -1,
        Some(c) => match c {
            ErrorCode::MissingXmpMetadata => 0,
            ErrorCode::MissingPdfaIdentification => 1,
            ErrorCode::InvalidPdfaIdentification => 2,
            ErrorCode::XmpMetadataMismatch => 3,
            ErrorCode::FontNotEmbedded => 4,
            ErrorCode::FontMissingTables => 5,
            ErrorCode::FontInvalidEncoding => 6,
            ErrorCode::FontMissingToUnicode => 7,
            ErrorCode::DeviceColorWithoutIntent => 8,
            ErrorCode::MissingOutputIntent => 9,
            ErrorCode::InvalidIccProfile => 10,
            ErrorCode::IccProfileVersionMismatch => 11,
            ErrorCode::UnsupportedImageCompression => 12,
            ErrorCode::InvalidImageColorSpace => 13,
            ErrorCode::LzwCompressionNotAllowed => 14,
            ErrorCode::MissingDocumentStructure => 15,
            ErrorCode::InvalidStructureTree => 16,
            ErrorCode::MissingLanguage => 17,
            ErrorCode::TransparencyNotAllowed => 18,
            ErrorCode::JavaScriptNotAllowed => 19,
            ErrorCode::MultimediaNotAllowed => 20,
            ErrorCode::ExternalContentNotAllowed => 21,
            ErrorCode::EncryptionNotAllowed => 22,
            ErrorCode::InvalidAnnotation => 23,
            ErrorCode::MissingAppearanceStream => 24,
            ErrorCode::InvalidAction => 25,
            ErrorCode::LaunchActionNotAllowed => 26,
            ErrorCode::EmbeddedFileNotAllowed => 27,
            ErrorCode::MissingAfRelationship => 28,
            ErrorCode::PostScriptNotAllowed => 29,
            ErrorCode::ReferenceXObjectNotAllowed => 30,
            ErrorCode::OptionalContentIssue => 31,
        },
    }
}

/// Build a Java `java/util/ArrayList` from a slice of `JObject`.
fn build_jobject_list<'local>(
    env: &mut jni::Env<'local>,
    items: &[JObject<'local>],
) -> Result<JObject<'local>, JniError> {
    let list_class = env.find_class(&JNIString::from("java/util/ArrayList"))?;
    let list_ctor = env.get_method_id(&list_class, &JNIString::from("<init>"), jni_sig!("(I)V"))?;
    let list_add =
        env.get_method_id(&list_class, &JNIString::from("add"), jni_sig!("(Ljava/lang/Object;)Z"))?;
    let list = unsafe {
        env.new_object_unchecked(
            &list_class,
            list_ctor,
            &[jni::sys::jvalue {
                i: items.len() as i32,
            }],
        )?
    };
    for item in items {
        unsafe {
            env.call_method_unchecked(
                &list,
                list_add,
                jni::signature::ReturnType::Primitive(jni::signature::Primitive::Boolean),
                &[jni::sys::jvalue { l: item.as_raw() }],
            )?;
        }
    }
    Ok(list)
}

/// `Java_fyi_oxide_pdf_PdfAConverter_nativeConvertToPdfA` — convert a
/// PDF byte array to PDF/A and return a `ConversionResult` with the
/// converted bytes.
#[no_mangle]
pub extern "system" fn Java_fyi_oxide_pdf_PdfAConverter_nativeConvertToPdfA<'local>(
    mut env: EnvUnowned<'local>,
    _class: JClass<'local>,
    input_bytes: JByteArray<'local>,
    level_ordinal: jint,
) -> jobject {
    env.with_env(|env| -> Result<jobject, JniError> {
        // Convert Java byte[] to Rust Vec<u8>
        let input_vec: Vec<u8> = env.convert_byte_array(&input_bytes)?;
        
        // Parse input bytes into a PdfDocument
        let mut doc = match PdfDocument::from_bytes(input_vec) {
            Ok(d) => d,
            Err(e) => {
                throw_pdf(env, &e)?;
                return Ok(std::ptr::null_mut());
            },
        };
        
        // Map level ordinal
        let level = map_pdfa_ordinal(env, level_ordinal)?;
        
        // Convert to PDF/A
        let result = match convert_to_pdf_a(&mut doc, level) {
            Ok(r) => r,
            Err(e) => {
                throw_pdf(env, &e)?;
                return Ok(std::ptr::null_mut());
            },
        };
        
        // Build Java ConversionResult with converted bytes
        build_conversion_result(env, &result, &doc.source_bytes)
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

/// Construct a Java `ConversionResult` from the Rust `ConversionResult`
/// and the converted document bytes.
fn build_conversion_result<'local>(
    env: &mut jni::Env<'local>,
    result: &RustConversionResult,
    converted_bytes: &[u8],
) -> Result<jobject, JniError> {
    // Find classes and method IDs.
    let cr_class = env.find_class(&JNIString::from("fyi/oxide/pdf/conversion/ConversionResult"))?;
    let cr_ctor = env.get_method_id(
        &cr_class,
        &JNIString::from("<init>"),
        jni_sig!("(ZLfyi/oxide/pdf/compliance/PdfALevel;Ljava/util/List;Ljava/util/List;[B)V"),
    )?;

    let pa_level_class = env.find_class(&JNIString::from("fyi/oxide/pdf/compliance/PdfALevel"))?;
    let at_class = env.find_class(&JNIString::from("fyi/oxide/pdf/conversion/ActionType"))?;
    let ce_class = env.find_class(&JNIString::from("fyi/oxide/pdf/conversion/ConversionError"))?;
    let ce_ctor = env.get_method_id(
        &ce_class,
        &JNIString::from("<init>"),
        jni_sig!("(Ljava/lang/String;Ljava/lang/String;)V"),
    )?;
    let ca_class = env.find_class(&JNIString::from("fyi/oxide/pdf/conversion/ConversionAction"))?;
    let ca_ctor = env.get_method_id(
        &ca_class,
        &JNIString::from("<init>"),
        jni_sig!("(Lfyi/oxide/pdf/conversion/ActionType;Ljava/lang/String;I)V"),
    )?;

    // Get the PdfALevel enum constant.
    let level_name = JNIString::from(pdfa_level_java_name(result.level));
    let level_obj = env
        .get_static_field(
            &pa_level_class,
            &level_name,
            jni_sig!("Lfyi/oxide/pdf/compliance/PdfALevel;"),
        )?
        .l()?;

    // Build ConversionAction list.
    let mut action_objs: Vec<JObject<'local>> = Vec::with_capacity(result.actions.len());
    for action in &result.actions {
        let at_name = JNIString::from(action_type_java_name(action.action_type));
        let at_obj = env
            .get_static_field(
                &at_class,
                &at_name,
                jni_sig!("Lfyi/oxide/pdf/conversion/ActionType;"),
            )?
            .l()?;
        let desc_str = env.new_string(&action.description)?;
        let fixed_code = error_code_to_int(action.fixed_error.as_ref());
        let action_obj = unsafe {
            env.new_object_unchecked(
                &ca_class,
                ca_ctor,
                &[
                    jni::sys::jvalue { l: at_obj.as_raw() },
                    jni::sys::jvalue {
                        l: desc_str.as_raw(),
                    },
                    jni::sys::jvalue { i: fixed_code },
                ],
            )?
        };
        action_objs.push(action_obj);
    }
    let actions_list = build_jobject_list(env, &action_objs)?;

    // Build ConversionError list.
    let mut error_objs: Vec<JObject<'local>> = Vec::with_capacity(result.errors.len());
    for err in &result.errors {
        let code_str = env.new_string(&err.error_code.to_string())?;
        let reason_str = env.new_string(&err.reason)?;
        let error_obj = unsafe {
            env.new_object_unchecked(
                &ce_class,
                ce_ctor,
                &[
                    jni::sys::jvalue {
                        l: code_str.as_raw(),
                    },
                    jni::sys::jvalue {
                        l: reason_str.as_raw(),
                    },
                ],
            )?
        };
        error_objs.push(error_obj);
    }
    let errors_list = build_jobject_list(env, &error_objs)?;

    // Convert converted_bytes to Java byte array
    let bytes_array = env.byte_array_from_slice(converted_bytes)?;

    // Construct ConversionResult with converted bytes.
    let result_obj = unsafe {
        env.new_object_unchecked(
            &cr_class,
            cr_ctor,
            &[
                jni::sys::jvalue {
                    i: if result.success { 1 } else { 0 },
                },
                jni::sys::jvalue {
                    l: level_obj.as_raw(),
                },
                jni::sys::jvalue {
                    l: actions_list.as_raw(),
                },
                jni::sys::jvalue {
                    l: errors_list.as_raw(),
                },
                jni::sys::jvalue {
                    l: bytes_array.as_raw(),
                },
            ],
        )?
    };
    Ok(result_obj.as_raw())
}
