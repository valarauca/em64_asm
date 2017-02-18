
#![allow(dead_code)]

pub type FeatureSet<'a> = &'a [Ext];


#[repr(u8)]
#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub enum Ext {

	//Default things
	X86,
	X64,


	//Intel EDX values CPUID EAX=1
	X87,
	TSC,
	CX8,
	SEP,
	CMOV,
	CLFSH,
	MMX,
	FXSR,
	SSE,
	SSE2,

	//Intel ECX values CPUID EAX=1
	SSE3,
	PCLMULQDQ,
	MONITOR,
	VMX,
	SMX,
	SSSE3,
	FMA3,
	CX16,
	SSE41,
	SSE42,
	MOVE,
	POPCNT,
	AES,
	XSAVE,
	AVX,
	F16C,
	RDRAND,

	//Intel EBX values CPU EAX=7
	SGX,
	BMI1,
	HLE,
	AVX2,
	BMI2,
	ERMS,
	INVPCID,
	RTM,
	MPX,
	AVX512_F,
	AVX512_DQ,
	RDSEED,
	ADX,
	AVX512_IFMA,
	PCOMIT,
	CLFLUSHOPT,
	CLWB,
	AVX512_PF,
	AVX512_ER,
	AVX512_CD,
	SHA,
	AVX512_BW,
	AVX512_VL,

	//Intel ECX values CPUID EAX=7
	PREFETCHWT1,
	AVX512_BMI,
	UMIP,
	AVX512_POPCNTDQ,
	
	//Intel EDX values CPUID EAX=7
	AVX512_4VNNIW,
	AVX512_4FMAPS
}
