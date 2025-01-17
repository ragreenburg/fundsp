//! The hacker prelude, a fully 64-bit environment for audio processing.

pub use super::audionode::*;
pub use super::audiounit::*;
pub use super::combinator::*;
pub use super::delay::*;
pub use super::dynamics::*;
pub use super::envelope::*;
pub use super::feedback::*;
pub use super::filter::*;
pub use super::fir::*;
pub use super::math::*;
pub use super::moog::*;
pub use super::net::*;
pub use super::noise::*;
pub use super::oscillator::*;
pub use super::oversample::*;
pub use super::pan::*;
pub use super::sequencer::*;
pub use super::shape::*;
pub use super::signal::*;
pub use super::svf::*;
pub use super::wave::*;
pub use super::wavetable::*;
pub use super::*;

pub use num_complex::Complex64;

// Combinator environment.
// We like to define all kinds of useful functions here.

// Import some typenum integers for reporting arities.
pub type U0 = numeric_array::typenum::U0;
pub type U1 = numeric_array::typenum::U1;
pub type U2 = numeric_array::typenum::U2;
pub type U3 = numeric_array::typenum::U3;
pub type U4 = numeric_array::typenum::U4;
pub type U5 = numeric_array::typenum::U5;
pub type U6 = numeric_array::typenum::U6;
pub type U7 = numeric_array::typenum::U7;
pub type U8 = numeric_array::typenum::U8;
pub type U9 = numeric_array::typenum::U9;
pub type U10 = numeric_array::typenum::U10;
pub type U11 = numeric_array::typenum::U11;
pub type U12 = numeric_array::typenum::U12;
pub type U13 = numeric_array::typenum::U13;
pub type U14 = numeric_array::typenum::U14;
pub type U15 = numeric_array::typenum::U15;
pub type U16 = numeric_array::typenum::U16;
pub type U17 = numeric_array::typenum::U17;
pub type U18 = numeric_array::typenum::U18;
pub type U19 = numeric_array::typenum::U19;
pub type U20 = numeric_array::typenum::U20;
pub type U21 = numeric_array::typenum::U21;
pub type U22 = numeric_array::typenum::U22;
pub type U23 = numeric_array::typenum::U23;
pub type U24 = numeric_array::typenum::U24;
pub type U25 = numeric_array::typenum::U25;
pub type U26 = numeric_array::typenum::U26;
pub type U27 = numeric_array::typenum::U27;
pub type U28 = numeric_array::typenum::U28;
pub type U29 = numeric_array::typenum::U29;
pub type U30 = numeric_array::typenum::U30;
pub type U31 = numeric_array::typenum::U31;
pub type U32 = numeric_array::typenum::U32;
pub type U33 = numeric_array::typenum::U33;
pub type U34 = numeric_array::typenum::U34;
pub type U35 = numeric_array::typenum::U35;
pub type U36 = numeric_array::typenum::U36;
pub type U37 = numeric_array::typenum::U37;
pub type U38 = numeric_array::typenum::U38;
pub type U39 = numeric_array::typenum::U39;
pub type U40 = numeric_array::typenum::U40;
pub type U41 = numeric_array::typenum::U41;
pub type U42 = numeric_array::typenum::U42;
pub type U43 = numeric_array::typenum::U43;
pub type U44 = numeric_array::typenum::U44;
pub type U45 = numeric_array::typenum::U45;
pub type U46 = numeric_array::typenum::U46;
pub type U47 = numeric_array::typenum::U47;
pub type U48 = numeric_array::typenum::U48;
pub type U49 = numeric_array::typenum::U49;
pub type U50 = numeric_array::typenum::U50;
pub type U51 = numeric_array::typenum::U51;
pub type U52 = numeric_array::typenum::U52;
pub type U53 = numeric_array::typenum::U53;
pub type U54 = numeric_array::typenum::U54;
pub type U55 = numeric_array::typenum::U55;
pub type U56 = numeric_array::typenum::U56;
pub type U57 = numeric_array::typenum::U57;
pub type U58 = numeric_array::typenum::U58;
pub type U59 = numeric_array::typenum::U59;
pub type U60 = numeric_array::typenum::U60;
pub type U61 = numeric_array::typenum::U61;
pub type U62 = numeric_array::typenum::U62;
pub type U63 = numeric_array::typenum::U63;
pub type U64 = numeric_array::typenum::U64;
pub type U65 = numeric_array::typenum::U65;
pub type U66 = numeric_array::typenum::U66;
pub type U67 = numeric_array::typenum::U67;
pub type U68 = numeric_array::typenum::U68;
pub type U69 = numeric_array::typenum::U69;
pub type U70 = numeric_array::typenum::U70;
pub type U71 = numeric_array::typenum::U71;
pub type U72 = numeric_array::typenum::U72;
pub type U73 = numeric_array::typenum::U73;
pub type U74 = numeric_array::typenum::U74;
pub type U75 = numeric_array::typenum::U75;
pub type U76 = numeric_array::typenum::U76;
pub type U77 = numeric_array::typenum::U77;
pub type U78 = numeric_array::typenum::U78;
pub type U79 = numeric_array::typenum::U79;
pub type U80 = numeric_array::typenum::U80;
pub type U81 = numeric_array::typenum::U81;
pub type U82 = numeric_array::typenum::U82;
pub type U83 = numeric_array::typenum::U83;
pub type U84 = numeric_array::typenum::U84;
pub type U85 = numeric_array::typenum::U85;
pub type U86 = numeric_array::typenum::U86;
pub type U87 = numeric_array::typenum::U87;
pub type U88 = numeric_array::typenum::U88;
pub type U89 = numeric_array::typenum::U89;
pub type U90 = numeric_array::typenum::U80;
pub type U91 = numeric_array::typenum::U91;
pub type U92 = numeric_array::typenum::U92;
pub type U93 = numeric_array::typenum::U93;
pub type U94 = numeric_array::typenum::U94;
pub type U95 = numeric_array::typenum::U95;
pub type U96 = numeric_array::typenum::U96;
pub type U97 = numeric_array::typenum::U97;
pub type U98 = numeric_array::typenum::U98;
pub type U99 = numeric_array::typenum::U99;
pub type U100 = numeric_array::typenum::U100;
pub type U101 = numeric_array::typenum::U101;
pub type U102 = numeric_array::typenum::U102;
pub type U103 = numeric_array::typenum::U103;
pub type U104 = numeric_array::typenum::U104;
pub type U105 = numeric_array::typenum::U105;
pub type U106 = numeric_array::typenum::U106;
pub type U107 = numeric_array::typenum::U107;
pub type U108 = numeric_array::typenum::U108;
pub type U109 = numeric_array::typenum::U109;
pub type U110 = numeric_array::typenum::U110;
pub type U111 = numeric_array::typenum::U111;
pub type U112 = numeric_array::typenum::U112;
pub type U113 = numeric_array::typenum::U113;
pub type U114 = numeric_array::typenum::U114;
pub type U115 = numeric_array::typenum::U115;
pub type U116 = numeric_array::typenum::U116;
pub type U117 = numeric_array::typenum::U117;
pub type U118 = numeric_array::typenum::U118;
pub type U119 = numeric_array::typenum::U119;
pub type U120 = numeric_array::typenum::U120;
pub type U121 = numeric_array::typenum::U121;
pub type U122 = numeric_array::typenum::U122;
pub type U123 = numeric_array::typenum::U123;
pub type U124 = numeric_array::typenum::U124;
pub type U125 = numeric_array::typenum::U125;
pub type U126 = numeric_array::typenum::U126;
pub type U127 = numeric_array::typenum::U127;
pub type U128 = numeric_array::typenum::U128;

/// Constant node.
/// Synonymous with `[dc]`.
/// - Output(s): constant
#[inline]
pub fn constant<X: ConstantFrame<Sample = f64>>(x: X) -> An<Constant<X::Size, f64>>
where
    X::Size: Size<f64>,
{
    An(Constant::new(x.convert()))
}

/// Constant node.
/// Synonymous with `constant`.
/// (DC stands for "direct current", which is an electrical engineering term used with signals.)
/// - Output(s): constant
#[inline]
pub fn dc<X: ConstantFrame<Sample = f64>>(x: X) -> An<Constant<X::Size, f64>>
where
    X::Size: Size<f64>,
{
    An(Constant::new(x.convert()))
}

/// Tagged constant. Outputs the (scalar) value of the tag.
/// - Output 0: value
#[inline]
pub fn tag(id: Tag, value: f64) -> An<Tagged<f64>> {
    An(Tagged::new(id, value))
}

/// Zero generator.
/// - Output 0: zero
#[inline]
pub fn zero() -> An<Constant<U1, f64>> {
    constant(0.0)
}

/// Multichannel zero generator.
/// - Output(s): zero
#[inline]
pub fn multizero<N: Size<f64>>() -> An<Constant<N, f64>> {
    An(Constant::new(Frame::splat(0.0)))
}

/// Mono pass-through.
#[inline]
pub fn pass() -> An<Pass<f64>> {
    An(Pass::new())
}

/// Multichannel pass-through.
#[inline]
pub fn multipass<N: Size<f64>>() -> An<MultiPass<N, f64>> {
    An(MultiPass::new())
}

/// Timer node. A node with no inputs or outputs that presents time as a parameter.
/// It can be added to any node by stacking.
#[inline]
pub fn timer(tag: Tag) -> An<Timer<f64>> {
    An(Timer::new(DEFAULT_SR, tag))
}

/// Monitor node. Passes through input and presents as a parameter
/// an aspect of the input signal according to the chosen metering mode.
/// -Input 0: input signal
/// -Output 0: input signal
#[inline]
pub fn monitor(meter: Meter, tag: Tag) -> An<Monitor<f64>> {
    An(Monitor::new(tag, DEFAULT_SR, meter))
}

/// Meter node.
/// Outputs a summary of the input according to the chosen metering mode.
/// -Input 0: input signal
/// -Output 0: input signal summary
#[inline]
pub fn meter(meter: Meter) -> An<MeterNode<f64>> {
    An(MeterNode::new(DEFAULT_SR, meter))
}

/// Mono sink. Input is discarded.
#[inline]
pub fn sink() -> An<Sink<U1, f64>> {
    An(Sink::new())
}

/// Multichannel sink. Inputs are discarded.
#[inline]
pub fn multisink<N: Size<f64>>() -> An<Sink<N, f64>> {
    An(Sink::new())
}

/// Swap stereo channels.
/// - Input 0: left channel.
/// - Input 1: right channel.
/// - Output 0: right channel input.
/// - Output 1: left channel input.
#[inline]
pub fn swap() -> An<Swap<f64>> {
    An(Swap::new())
}

/// Sine oscillator.
/// - Input 0: frequency (Hz)
/// - Output 0: sine wave
#[inline]
pub fn sine() -> An<Sine<f64>> {
    An(Sine::new(DEFAULT_SR))
}

/// Fixed sine oscillator at `f` Hz.
/// - Output 0: sine wave
#[inline]
pub fn sine_hz(f: f64) -> An<Pipe<f64, Constant<U1, f64>, Sine<f64>>> {
    super::prelude::sine_hz(f)
}

/// Add constant to signal.
#[inline]
pub fn add<X: ConstantFrame<Sample = f64>>(
    x: X,
) -> An<Binop<f64, FrameAdd<X::Size, f64>, MultiPass<X::Size, f64>, Constant<X::Size, f64>>>
where
    X::Size: Size<f64> + Add<U0>,
    <X::Size as Add<U0>>::Output: Size<f64>,
{
    An(MultiPass::<X::Size, f64>::new()) + dc(x)
}

/// Subtract constant from signal.
#[inline]
pub fn sub<X: ConstantFrame<Sample = f64>>(
    x: X,
) -> An<Binop<f64, FrameSub<X::Size, f64>, MultiPass<X::Size, f64>, Constant<X::Size, f64>>>
where
    X::Size: Size<f64> + Add<U0>,
    <X::Size as Add<U0>>::Output: Size<f64>,
{
    An(MultiPass::<X::Size, f64>::new()) - dc(x)
}

/// Multiply signal with constant.
#[inline]
pub fn mul<X: ConstantFrame<Sample = f64>>(
    x: X,
) -> An<Binop<f64, FrameMul<X::Size, f64>, MultiPass<X::Size, f64>, Constant<X::Size, f64>>>
where
    X::Size: Size<f64> + Add<U0>,
    <X::Size as Add<U0>>::Output: Size<f64>,
{
    An(MultiPass::<X::Size, f64>::new()) * dc(x)
}

/// Butterworth lowpass filter (2nd order).
/// - Input 0: audio
/// - Input 1: cutoff frequency (Hz)
/// - Output 0: filtered audio
#[inline]
pub fn butterpass() -> An<ButterLowpass<f64, f64, U2>> {
    An(ButterLowpass::new(DEFAULT_SR, 440.0))
}

/// Butterworth lowpass filter (2nd order) with fixed cutoff frequency `f` Hz.
/// - Input 0: audio
/// - Output 0: filtered audio
#[inline]
pub fn butterpass_hz(f: f64) -> An<ButterLowpass<f64, f64, U1>> {
    super::prelude::butterpass_hz(f)
}

/// One-pole lowpass filter (1st order).
/// - Input 0: audio
/// - Input 1: cutoff frequency (Hz)
/// - Output 0: filtered audio
#[inline]
pub fn lowpole() -> An<Lowpole<f64, f64, U2>> {
    An(Lowpole::new(DEFAULT_SR, 440.0))
}

/// One-pole lowpass filter (1st order) with fixed cutoff frequency `f` Hz.
/// - Input 0: audio
/// - Output 0: filtered audio
#[inline]
pub fn lowpole_hz(f: f64) -> An<Lowpole<f64, f64, U1>> {
    super::prelude::lowpole_hz(f)
}

/// Allpass filter with adjustable delay (delay > 0) in samples at DC.
/// - Input 0: audio
/// - Input 1: delay in samples
/// - Output 0: filtered audio
#[inline]
pub fn allpole() -> An<Allpole<f64, f64, U2>> {
    An(Allpole::new(DEFAULT_SR, 1.0))
}

/// Allpass filter with delay (delay > 0) in samples at DC.
/// - Input 0: audio
/// - Output 0: filtered audio
#[inline]
pub fn allpole_delay(delay_in_samples: f64) -> An<Allpole<f64, f64, U1>> {
    An(Allpole::new(DEFAULT_SR, delay_in_samples))
}

/// One-pole, one-zero highpass filter (1st order).
/// - Input 0: audio
/// - Input 1: cutoff frequency (Hz)
/// - Output 0: filtered audio
#[inline]
pub fn highpole() -> An<Highpole<f64, f64, U2>> {
    An(Highpole::new(DEFAULT_SR, 440.0))
}

/// One-pole, one-zero highpass filter (1st order) with fixed cutoff frequency f.
/// - Input 0: audio
/// - Output 0: filtered audio
#[inline]
pub fn highpole_hz(f: f64) -> An<Highpole<f64, f64, U1>> {
    An(Highpole::new(DEFAULT_SR, f))
}

/// Constant-gain bandpass resonator.
/// - Input 0: audio
/// - Input 1: cutoff frequency (Hz)
/// - Input 2: bandwidth (Hz)
/// - Output 0: filtered audio
#[inline]
pub fn resonator() -> An<Resonator<f64, f64, U3>> {
    An(Resonator::new(DEFAULT_SR, 440.0, 110.0))
}

/// Constant-gain bandpass resonator with fixed `center` frequency (Hz) and `bandwidth` (Hz).
/// - Input 0: audio
/// - Output 0: filtered audio
#[inline]
pub fn resonator_hz(center: f64, bandwidth: f64) -> An<Resonator<f64, f64, U1>> {
    super::prelude::resonator_hz(center, bandwidth)
}

/// Moog resonant lowpass filter.
/// - Input 0: input signal
/// - Input 1: cutoff frequency (Hz)
/// - Input 2: Q
/// - Output 0: filtered signal
#[inline]
pub fn moog() -> An<Moog<f64, f64, U3>> {
    An(Moog::new(DEFAULT_SR, 1000.0, 0.1))
}

/// Moog resonant lowpass filter with fixed Q.
/// - Input 0: input signal
/// - Input 1: cutoff frequency (Hz)
/// - Output 0: filtered signal
#[inline]
pub fn moog_q(
    q: f64,
) -> An<Pipe<f64, Stack<f64, MultiPass<U2, f64>, Constant<U1, f64>>, Moog<f64, f64, U3>>> {
    (multipass::<U2>() | dc(q)) >> An(Moog::new(convert(DEFAULT_SR), 1000.0, q))
}

/// Moog resonant lowpass filter with fixed cutoff frequency and Q.
/// - Input 0: input signal
/// - Output 0: filtered signal
#[inline]
pub fn moog_hz(frequency: f64, q: f64) -> An<Moog<f64, f64, U1>> {
    An(Moog::new(DEFAULT_SR, frequency, q))
}

/// Morphing filter that morphs between lowpass, peak and highpass modes.
/// - Input 0: input signal
/// - Input 1: center frequency (Hz)
/// - Input 2: Q
/// - Input 3: morph in -1...1 (-1 = lowpass, 0 = peak, 1 = highpass)
/// - Output 0: filtered signal
pub fn morph() -> An<super::prelude::Morph<f64, f64>> {
    super::prelude::morph()
}

/// Morphing filter with center frequency `f`, Q value `q`, and morph `morph`
/// (-1 = lowpass, 0 = peaking, 1 = highpass).
/// - Input 0: input signal
/// - Output 0: filtered signal
pub fn morph_hz(
    f: f64,
    q: f64,
    morph: f64,
) -> An<Pipe<f64, Stack<f64, Pass<f64>, Constant<U3, f64>>, super::prelude::Morph<f64, f64>>> {
    super::prelude::morph_hz(f, q, morph)
}

/// Control envelope from time-varying function `f(t)` with `t` in seconds.
/// Spaces samples using pseudorandom jittering.
/// Synonymous with `lfo`.
/// - Output(s): envelope linearly interpolated from samples at 2 ms intervals (average).
#[inline]
pub fn envelope<E, R>(f: E) -> An<Envelope<f64, f64, E, R>>
where
    E: Fn(f64) -> R,
    R: ConstantFrame<Sample = f64>,
    R::Size: Size<f64>,
{
    An(Envelope::new(0.002, DEFAULT_SR, f))
}

/// Control envelope from time-varying function `f(t)` with `t` in seconds.
/// Spaces samples using pseudorandom jittering.
/// Synonymous with `envelope`.
/// - Output(s): envelope linearly interpolated from samples at 2 ms intervals (average).
#[inline]
pub fn lfo<E, R>(f: E) -> An<Envelope<f64, f64, E, R>>
where
    E: Fn(f64) -> R,
    R: ConstantFrame<Sample = f64>,
    R::Size: Size<f64>,
{
    An(Envelope::new(0.002, DEFAULT_SR, f))
}

/// Control envelope from time-varying, input dependent function `f(t, x)` with `t` in seconds.
/// Spaces samples using pseudorandom jittering.
/// Synonymous with `lfo2`.
/// - Input 0: x
/// - Output(s): envelope linearly interpolated from samples at 2 ms intervals (average).
#[inline]
pub fn envelope2<E, R>(f: E) -> An<Envelope2<f64, f64, E, R>>
where
    E: Fn(f64, f64) -> R,
    R: ConstantFrame<Sample = f64>,
    R::Size: Size<f64>,
{
    An(Envelope2::new(0.002, DEFAULT_SR, f))
}

/// Control envelope from time-varying, input dependent function `f(t, x)` with `t` in seconds.
/// Spaces samples using pseudorandom jittering.
/// Synonymous with `envelope2`.
/// - Input 0: x
/// - Output(s): envelope linearly interpolated from samples at 2 ms intervals (average).
#[inline]
pub fn lfo2<E, R>(f: E) -> An<Envelope2<f64, f64, E, R>>
where
    E: Fn(f64, f64) -> R,
    R: ConstantFrame<Sample = f64>,
    R::Size: Size<f64>,
{
    An(Envelope2::new(0.002, DEFAULT_SR, f))
}

/// Control envelope from time-varying, input dependent function `f(t, x, y)` with `t` in seconds.
/// Spaces samples using pseudorandom jittering.
/// Synonymous with `lfo3`.
/// - Input 0: x
/// - Input 1: y
/// - Output(s): envelope linearly interpolated from samples at 2 ms intervals (average).
#[inline]
pub fn envelope3<E, R>(f: E) -> An<Envelope3<f64, f64, E, R>>
where
    E: Fn(f64, f64, f64) -> R,
    R: ConstantFrame<Sample = f64>,
    R::Size: Size<f64>,
{
    An(Envelope3::new(0.002, DEFAULT_SR, f))
}

/// Control envelope from time-varying, input dependent function `f(t, x, y)` with `t` in seconds.
/// Spaces samples using pseudorandom jittering.
/// Synonymous with `envelope3`.
/// - Input 0: x
/// - Input 1: y
/// - Output(s): envelope linearly interpolated from samples at 2 ms intervals (average).
#[inline]
pub fn lfo3<E, R>(f: E) -> An<Envelope3<f64, f64, E, R>>
where
    E: Fn(f64, f64, f64) -> R,
    R: ConstantFrame<Sample = f64>,
    R::Size: Size<f64>,
{
    An(Envelope3::new(0.002, DEFAULT_SR, f))
}

/// Maximum Length Sequence noise generator from an `n`-bit sequence.
/// - Output 0: repeating white noise sequence of only -1 and 1 values.
#[inline]
pub fn mls_bits(n: i64) -> An<Mls<f64>> {
    An(Mls::new(MlsState::new(n as u32)))
}

/// Default Maximum Length Sequence noise generator.
/// - Output 0: repeating white noise sequence of only -1 and 1 values.
#[inline]
pub fn mls() -> An<Mls<f64>> {
    mls_bits(29)
}

/// White noise generator.
/// Synonymous with `white`.
/// - Output 0: white noise.
#[inline]
pub fn noise() -> An<Noise<f64>> {
    An(Noise::new())
}

/// White noise generator.
/// Synonymous with `noise`.
/// - Output 0: white noise.
#[inline]
pub fn white() -> An<Noise<f64>> {
    An(Noise::new())
}

/// FIR filter.
/// - Input 0: signal.
/// - Output 0: filtered signal.
#[inline]
pub fn fir<X: ConstantFrame<Sample = f64>>(weights: X) -> An<Fir<f64, X::Size>> {
    An(Fir::new(weights))
}

/// Single sample delay.
/// - Input 0: signal.
/// - Output 0: delayed signal.
#[inline]
pub fn tick() -> An<Tick<U1, f64>> {
    An(Tick::new(DEFAULT_SR))
}

/// Multichannel single sample delay.
/// - Inputs: signal.
/// - Outputs: delayed signal.
pub fn multitick<N: Size<f64>>() -> An<Tick<N, f64>> {
    An(Tick::new(convert(DEFAULT_SR)))
}

/// Fixed delay of `t` seconds.
/// Delay time is rounded to the nearest sample.
/// - Input 0: signal.
/// - Output 0: delayed signal.
#[inline]
pub fn delay(t: f64) -> An<Delay<f64>> {
    An(Delay::new(t, DEFAULT_SR))
}

/// Tapped delay line with cubic interpolation.
/// Minimum and maximum delay times are in seconds.
/// - Input 0: signal.
/// - Input 1: delay time in seconds.
/// - Output 0: delayed signal.
pub fn tap(min_delay: f64, max_delay: f64) -> An<Tap<U1, f64>> {
    An(Tap::new(DEFAULT_SR, min_delay, max_delay))
}

/// Tapped delay line with cubic interpolation.
/// The number of taps is `N`.
/// Minimum and maximum delay times are in seconds.
/// - Input 0: signal.
/// - Inputs 1...N: delay time in seconds.
/// - Output 0: delayed signal.
pub fn multitap<N>(min_delay: f64, max_delay: f64) -> An<Tap<N, f64>>
where
    N: Size<f64> + Add<U1>,
    <N as Add<U1>>::Output: Size<f64>,
{
    An(Tap::new(DEFAULT_SR, min_delay, max_delay))
}

/// 2x oversample enclosed `node`.
/// - Inputs and outputs: from `node`.
pub fn oversample<X>(node: An<X>) -> An<Oversampler<f64, X>>
where
    X: AudioNode<Sample = f64>,
    X::Inputs: Size<f64>,
    X::Outputs: Size<f64>,
    X::Inputs: Size<Frame<f64, U128>>,
    X::Outputs: Size<Frame<f64, U128>>,
{
    An(Oversampler::new(DEFAULT_SR, node.0))
}

/// Mix output of enclosed circuit `node` back to its input.
/// Feedback circuit `node` must have an equal number of inputs and outputs.
/// - Inputs: input signal.
/// - Outputs: `node` output signal.
#[inline]
pub fn feedback<N, X>(node: An<X>) -> An<Feedback<N, f64, X, FrameId<N, f64>>>
where
    X: AudioNode<Sample = f64, Inputs = N, Outputs = N>,
    X::Inputs: Size<f64>,
    X::Outputs: Size<f64>,
    N: Size<f64>,
{
    An(Feedback::new(node.0, FrameId::new()))
}

/// Transform channels freely. Accounted as non-linear processing for signal flow.
///
/// # Example
/// ```
/// # use fundsp::hacker::*;
/// let my_max = map(|i: &Frame<f64, U2>| max(i[0], i[1]));
/// ```
#[inline]
pub fn map<M, I, O>(f: M) -> An<Map<f64, M, I, O>>
where
    M: Fn(&Frame<f64, I>) -> O,
    I: Size<f64>,
    O: ConstantFrame<Sample = f64>,
    O::Size: Size<f64>,
{
    An(Map::new(f, Routing::Arbitrary))
}

/// Keeps a signal zero centered.
/// Filter cutoff `c` is usually somewhere below the audible range.
/// The default blocker cutoff is 10 Hz.
/// - Input 0: input signal
/// - Output 0: signal with DC removed.
#[inline]
pub fn dcblock_hz(c: f64) -> An<DCBlock<f64, f64>> {
    An(DCBlock::new(DEFAULT_SR, c))
}

/// Keeps a signal zero centered.
/// - Input 0: input signal
/// - Output 0: signal with DC removed.
#[inline]
pub fn dcblock() -> An<DCBlock<f64, f64>> {
    dcblock_hz(10.0)
}

/// Apply 10 ms of fade-in to signal at time zero.
/// - Input 0: input signal
/// - Output 0: signal with fade-in.
#[inline]
pub fn declick() -> An<Declick<f64, f64>> {
    super::prelude::declick()
}

/// Apply `t` seconds of fade-in to signal at time zero.
/// - Input 0: input signal
/// - Output 0: signal with fade-in.
#[inline]
pub fn declick_s(t: f64) -> An<Declick<f64, f64>> {
    super::prelude::declick_s(t)
}

/// Shape signal with a waveshaper function.
/// - Input 0: input signal
/// - Output 0: shaped signal
#[inline]
pub fn shape_fn<S: Fn(f64) -> f64>(f: S) -> An<ShaperFn<f64, S>> {
    super::prelude::shape_fn(f)
}

/// Shape signal according to shaping mode.
/// - Input 0: input signal
/// - Output 0: shaped signal
#[inline]
pub fn shape(mode: Shape<f64>) -> An<Shaper<f64>> {
    super::prelude::shape(mode)
}

/// Clip signal to -1...1.
/// - Input 0: input signal
/// - Output 0: clipped signal
#[inline]
pub fn clip() -> An<Shaper<f64>> {
    super::prelude::clip()
}

/// Clip signal to min...max.
/// - Input 0: input signal
/// - Output 0: clipped signal
#[inline]
pub fn clip_to(minimum: f64, maximum: f64) -> An<Shaper<f64>> {
    super::prelude::clip_to(minimum, maximum)
}

/// Equal power mono-to-stereo panner.
/// - Input 0: input signal
/// - Input 1: pan in -1...1 (left to right).
/// - Output 0: left channel
/// - Output 1: right channel
#[inline]
pub fn panner() -> An<Panner<f64, U2>> {
    An(Panner::new(0.0))
}

/// Fixed equal power mono-to-stereo panner with pan value in -1...1.
/// - Input 0: input signal
/// - Output 0: left channel
/// - Output 1: right channel
#[inline]
pub fn pan(pan: f64) -> An<Panner<f64, U1>> {
    An(Panner::new(pan))
}

/// Parameter follower filter with halfway response time `t` seconds.
/// - Input 0: input signal
/// - Output 0: smoothed signal
#[inline]
pub fn follow<S: ScalarOrPair<Sample = f64>>(t: S) -> An<AFollow<f64, f64, S>> {
    An(AFollow::new(DEFAULT_SR, t))
}

/// Look-ahead limiter with `(attack, release)` times in seconds.
/// Look-ahead is equal to the attack time.
/// - Input 0: input signal
/// - Output 0: signal limited to -1...1
#[inline]
pub fn limiter<S: ScalarOrPair<Sample = f64>>(time: S) -> An<Limiter<f64, U1, S>> {
    An(Limiter::new(DEFAULT_SR, time))
}

/// Stereo look-ahead limiter with `(attack, release)` times in seconds.
/// Look-ahead is equal to the attack time.
/// - Input 0: left input signal
/// - Input 1: right input signal
/// - Output 0: left signal limited to -1...1
/// - Output 0: right signal limited to -1...1
#[inline]
pub fn limiter_stereo<S: ScalarOrPair<Sample = f64>>(time: S) -> An<Limiter<f64, U2, S>> {
    An(Limiter::new(DEFAULT_SR, time))
}

/// Pinking filter.
/// - Input 0: input signal
/// - Output 0: filtered signal
#[inline]
pub fn pinkpass() -> An<Pinkpass<f64, f64>> {
    An(Pinkpass::new(DEFAULT_SR))
}

/// Pink noise.
/// - Output 0: pink noise
#[inline]
pub fn pink() -> An<Pipe<f64, Noise<f64>, Pinkpass<f64, f64>>> {
    super::prelude::pink()
}

/// Brown noise.
/// - Output 0: brown noise
#[inline]
pub fn brown() -> An<
    Pipe<f64, Noise<f64>, Binop<f64, FrameMul<U1, f64>, Lowpole<f64, f64, U1>, Constant<U1, f64>>>,
> {
    // Empirical normalization factor.
    white() >> lowpole_hz(10.0) * dc(13.7)
}

/// Frequency detector.
/// - Input 0: input signal
/// - Input 1: frequency to detect (Hz)
/// - Output 0: signal power at the chosen frequency
#[inline]
pub fn goertzel() -> An<Goertzel<f64, f64>> {
    An(Goertzel::new(DEFAULT_SR))
}

/// Frequency detector of frequency `f` Hz.
/// - Input 0: input signal
/// - Output 0: signal power at the chosen frequency
#[inline]
pub fn goertzel_hz(
    f: f64,
) -> An<Pipe<f64, Stack<f64, Pass<f64>, Constant<U1, f64>>, Goertzel<f64, f64>>> {
    super::prelude::goertzel_hz(f)
}

/// Feedback delay network.
/// Mix output of enclosed circuit `x` back to its input.
/// The output is diffused with a Hadamard matrix for feedback.
/// Feedback circuit `x` must have an equal number of inputs and outputs.
/// - Inputs: input signal.
/// - Outputs: `x` output signal.
#[inline]
pub fn fdn<N, X>(x: An<X>) -> An<Feedback<N, f64, X, FrameHadamard<N, f64>>>
where
    X: AudioNode<Sample = f64, Inputs = N, Outputs = N>,
    X::Inputs: Size<f64>,
    X::Outputs: Size<f64>,
    N: Size<f64>,
{
    An(Feedback::new(x.0, FrameHadamard::new()))
}

/// Bus `N` similar nodes from indexed generator `f`.
#[inline]
pub fn bus<N, X, F>(f: F) -> An<MultiBus<N, f64, X>>
where
    N: Size<f64>,
    N: Size<X>,
    X: AudioNode<Sample = f64>,
    X::Inputs: Size<f64>,
    X::Outputs: Size<f64>,
    F: Fn(i64) -> An<X>,
{
    super::prelude::bus(f)
}

/// Bus `N` similar nodes from fractional generator `f`.
/// The fractional generator is given values in the range 0...1.
#[inline]
pub fn busf<N, X, F>(f: F) -> An<MultiBus<N, f64, X>>
where
    N: Size<f64>,
    N: Size<X>,
    X: AudioNode<Sample = f64>,
    X::Inputs: Size<f64>,
    X::Outputs: Size<f64>,
    F: Fn(f64) -> An<X>,
{
    super::prelude::busf(f)
}

/// Stack `N` similar nodes from indexed generator `f`.
#[inline]
pub fn stack<N, X, F>(f: F) -> An<MultiStack<N, f64, X>>
where
    N: Size<f64>,
    N: Size<X>,
    X: AudioNode<Sample = f64>,
    X::Inputs: Size<f64> + Mul<N>,
    X::Outputs: Size<f64> + Mul<N>,
    <X::Inputs as Mul<N>>::Output: Size<f64>,
    <X::Outputs as Mul<N>>::Output: Size<f64>,
    F: Fn(i64) -> An<X>,
{
    super::prelude::stack(f)
}

/// Stack `N` similar nodes from fractional generator `f`.
/// The fractional generator is given values in the range 0...1.
#[inline]
pub fn stackf<N, X, F>(f: F) -> An<MultiStack<N, f64, X>>
where
    N: Size<f64>,
    N: Size<X>,
    X: AudioNode<Sample = f64>,
    X::Inputs: Size<f64> + Mul<N>,
    X::Outputs: Size<f64> + Mul<N>,
    <X::Inputs as Mul<N>>::Output: Size<f64>,
    <X::Outputs as Mul<N>>::Output: Size<f64>,
    F: Fn(f64) -> An<X>,
{
    super::prelude::stackf(f)
}

/// Branch into `N` similar nodes from indexed generator `f`.
#[inline]
pub fn branch<N, X, F>(f: F) -> An<MultiBranch<N, f64, X>>
where
    N: Size<f64>,
    N: Size<X>,
    X: AudioNode<Sample = f64>,
    X::Inputs: Size<f64>,
    X::Outputs: Size<f64> + Mul<N>,
    <X::Outputs as Mul<N>>::Output: Size<f64>,
    F: Fn(i64) -> An<X>,
{
    super::prelude::branch(f)
}

/// Branch into `N` similar nodes from fractional generator `f`.
/// The fractional generator is given values in the range 0...1.
#[inline]
pub fn branchf<N, X, F>(f: F) -> An<MultiBranch<N, f64, X>>
where
    N: Size<f64>,
    N: Size<X>,
    X: AudioNode<Sample = f64>,
    X::Inputs: Size<f64>,
    X::Outputs: Size<f64> + Mul<N>,
    <X::Outputs as Mul<N>>::Output: Size<f64>,
    F: Fn(f64) -> An<X>,
{
    super::prelude::branchf(f)
}

/// Mix together `N` similar nodes from indexed generator `f`.
#[inline]
pub fn sum<N, X, F>(f: F) -> An<Reduce<N, f64, X, FrameAdd<X::Outputs, f64>>>
where
    N: Size<f64>,
    N: Size<X>,
    X: AudioNode<Sample = f64>,
    X::Inputs: Size<f64> + Mul<N>,
    X::Outputs: Size<f64>,
    <X::Inputs as Mul<N>>::Output: Size<f64>,
    F: Fn(i64) -> An<X>,
{
    super::prelude::sum(f)
}

/// Mix together `N` similar nodes from fractional generator `f`.
/// The fractional generator is given values in the range 0...1.
#[inline]
pub fn sumf<N, X, F>(f: F) -> An<Reduce<N, f64, X, FrameAdd<X::Outputs, f64>>>
where
    N: Size<f64>,
    N: Size<X>,
    X: AudioNode<Sample = f64>,
    X::Inputs: Size<f64> + Mul<N>,
    X::Outputs: Size<f64>,
    <X::Inputs as Mul<N>>::Output: Size<f64>,
    F: Fn(f64) -> An<X>,
{
    super::prelude::sumf(f)
}

/// Chain together `N` similar nodes from indexed generator `f`.
#[inline]
pub fn pipe<N, X, F>(f: F) -> An<Chain<N, f64, X>>
where
    N: Size<f64>,
    N: Size<X>,
    X: AudioNode<Sample = f64>,
    X::Inputs: Size<f64>,
    X::Outputs: Size<f64>,
    F: Fn(i64) -> An<X>,
{
    super::prelude::pipe(f)
}

/// Chain together `N` similar nodes from fractional generator `f`.
/// The fractional generator is given values in the range 0...1.
#[inline]
pub fn pipef<N, X, F>(f: F) -> An<Chain<N, f64, X>>
where
    N: Size<f64>,
    N: Size<X>,
    X: AudioNode<Sample = f64>,
    X::Inputs: Size<f64>,
    X::Outputs: Size<f64>,
    F: Fn(f64) -> An<X>,
{
    super::prelude::pipef(f)
}

/// Split signal into N channels.
#[inline]
pub fn split<N>() -> An<Split<N, f64>>
where
    N: Size<f64>,
{
    super::prelude::split::<N, f64>()
}

/// Split M channels into N branches. The output has M * N channels.
#[inline]
pub fn multisplit<M, N>() -> An<MultiSplit<M, N, f64>>
where
    M: Size<f64> + Mul<N>,
    N: Size<f64>,
    <M as Mul<N>>::Output: Size<f64>,
{
    super::prelude::multisplit::<M, N, f64>()
}

/// Average N channels into one. Inverse of `split`.
#[inline]
pub fn join<N>() -> An<Join<N, f64>>
where
    N: Size<f64>,
{
    super::prelude::join::<N, f64>()
}

/// Average `N` branches of `M` channels into one branch with `M` channels.
/// The input has `M` * `N` channels. Inverse of `multisplit::<M, N>`.
#[inline]
pub fn multijoin<M, N>() -> An<MultiJoin<M, N, f64>>
where
    M: Size<f64> + Mul<N>,
    N: Size<f64>,
    <M as Mul<N>>::Output: Size<f64>,
{
    super::prelude::multijoin::<M, N, f64>()
}

/// Stereo reverb.
/// `wet` in 0...1 is balance of reverb mixed in, for example, 0.1.
/// `time` is approximate reverberation time to -60 dB in seconds.
/// - Input 0: left input signal
/// - Input 1: right input signal
/// - Output 0: reverberated left signal
/// - Output 1: reverberated right signal
pub fn reverb_stereo(
    wet: f64,
    time: f64,
) -> An<
    Bus<
        f64,
        Pipe<
            f64,
            Pipe<
                f64,
                Pipe<
                    f64,
                    MultiSplit<U2, U16, f64>,
                    Feedback<
                        U32,
                        f64,
                        MultiStack<
                            U32,
                            f64,
                            Pipe<
                                f64,
                                Pipe<f64, Pipe<f64, Delay<f64>, Fir<f64, U2>>, DCBlock<f64, f64>>,
                                Binop<
                                    f64,
                                    FrameMul<U1, f64>,
                                    MultiPass<U1, f64>,
                                    Constant<U1, f64>,
                                >,
                            >,
                        >,
                        FrameHadamard<U32, f64>,
                    >,
                >,
                MultiJoin<U2, U16, f64>,
            >,
            Binop<f64, FrameMul<U2, f64>, MultiPass<U2, f64>, Constant<U2, f64>>,
        >,
        Binop<f64, FrameMul<U2, f64>, MultiPass<U2, f64>, Constant<U2, f64>>,
    >,
> {
    super::prelude::reverb_stereo::<f64, f64>(wet, time)
}

/// Saw-like discrete summation formula oscillator.
/// - Input 0: frequency in Hz
/// - Input 1: roughness in 0...1 is the attenuation of successive partials.
/// - Output 0: DSF wave
pub fn dsf_saw() -> An<Dsf<f64, U2>> {
    An(Dsf::new(DEFAULT_SR, 1.0, 0.5))
}

/// Saw-like discrete summation formula oscillator.
/// Roughness in 0...1 is the attenuation of successive partials.
/// - Input 0: frequency in Hz
/// - Output 0: DSF wave
pub fn dsf_saw_r(roughness: f64) -> An<Dsf<f64, U1>> {
    An(Dsf::new(DEFAULT_SR, 1.0, roughness))
}

/// Square-like discrete summation formula oscillator.
/// - Input 0: frequency in Hz
/// - Input 1: roughness in 0...1 is the attenuation of successive partials.
/// - Output 0: DSF wave
pub fn dsf_square() -> An<Dsf<f64, U2>> {
    An(Dsf::new(DEFAULT_SR, 2.0, 0.5))
}

/// Square-like discrete summation formula oscillator.
/// Roughness in 0...1 is the attenuation of successive partials.
/// - Input 0: frequency in Hz
/// - Output 0: DSF wave
pub fn dsf_square_r(roughness: f64) -> An<Dsf<f64, U1>> {
    An(Dsf::new(DEFAULT_SR, 2.0, roughness))
}

/// Karplus-Strong plucked string oscillator with `frequency` in Hz.
/// High frequency damping is in 0...1.
/// - Input 0: string excitation
/// - Output 0: oscillator output
pub fn pluck(frequency: f64, gain_per_second: f64, high_frequency_damping: f64) -> An<Pluck<f64>> {
    An(Pluck::new(
        DEFAULT_SR,
        frequency,
        gain_per_second,
        high_frequency_damping,
    ))
}

/// Saw wave oscillator.
/// - Input 0: frequency in Hz
/// - Output 0: saw wave
#[inline]
pub fn saw() -> An<WaveSynth<'static, f64, U1>> {
    An(WaveSynth::new(DEFAULT_SR, &SAW_TABLE))
}

/// Square wave oscillator.
/// - Input 0: frequency in Hz
/// - Output 0: square wave
#[inline]
pub fn square() -> An<WaveSynth<'static, f64, U1>> {
    An(WaveSynth::new(DEFAULT_SR, &SQUARE_TABLE))
}

/// Triangle wave oscillator.
/// - Input 0: frequency in Hz
/// - Output 0: triangle wave
#[inline]
pub fn triangle() -> An<WaveSynth<'static, f64, U1>> {
    An(WaveSynth::new(DEFAULT_SR, &TRIANGLE_TABLE))
}

/// Fixed saw wave oscillator at `f` Hz.
/// - Output 0: saw wave
#[inline]
pub fn saw_hz(f: f64) -> An<Pipe<f64, Constant<U1, f64>, WaveSynth<'static, f64, U1>>> {
    super::prelude::saw_hz(f)
}

/// Fixed square wave oscillator at `f` Hz.
/// - Output 0: square wave
#[inline]
pub fn square_hz(f: f64) -> An<Pipe<f64, Constant<U1, f64>, WaveSynth<'static, f64, U1>>> {
    super::prelude::square_hz(f)
}

/// Fixed triangle wave oscillator at `f` Hz.
/// - Output 0: triangle wave
#[inline]
pub fn triangle_hz(f: f64) -> An<Pipe<f64, Constant<U1, f64>, WaveSynth<'static, f64, U1>>> {
    super::prelude::triangle_hz(f)
}

/// Lowpass filter.
/// - Input 0: audio
/// - Input 1: cutoff frequency (Hz)
/// - Input 2: Q
/// - Output 0: filtered audio
#[inline]
pub fn lowpass() -> An<Svf<f64, f64, LowpassMode<f64>>> {
    super::prelude::lowpass()
}

/// Lowpass filter with cutoff frequency `f` Hz with Q value `q`.
/// - Input 0: audio
/// - Output 0: filtered audio
#[inline]
pub fn lowpass_hz(f: f64, q: f64) -> An<FixedSvf<f64, f64, LowpassMode<f64>>> {
    super::prelude::lowpass_hz::<f64, f64>(f, q)
}

/// Lowpass filter with Q value `q`.
/// - Input 0: audio
/// - Input 1: cutoff frequency (Hz)
/// - Output 0: filtered audio
#[inline]
pub fn lowpass_q(
    q: f64,
) -> An<Pipe<f64, Stack<f64, MultiPass<U2, f64>, Constant<U1, f64>>, Svf<f64, f64, LowpassMode<f64>>>>
{
    super::prelude::lowpass_q::<f64, f64>(q)
}

/// Highpass filter.
/// - Input 0: audio
/// - Input 1: cutoff frequency (Hz)
/// - Input 2: Q
/// - Output 0: filtered audio
#[inline]
pub fn highpass() -> An<Svf<f64, f64, HighpassMode<f64>>> {
    super::prelude::highpass()
}

/// Highpass filter with cutoff frequency `f` Hz with Q value `q`.
/// - Input 0: audio
/// - Output 0: filtered audio
#[inline]
pub fn highpass_hz(f: f64, q: f64) -> An<FixedSvf<f64, f64, HighpassMode<f64>>> {
    super::prelude::highpass_hz::<f64, f64>(f, q)
}

/// Highpass filter with Q value `q`.
/// - Input 0: audio
/// - Input 1: cutoff frequency (Hz)
/// - Output 0: filtered audio
#[inline]
pub fn highpass_q(
    q: f64,
) -> An<
    Pipe<f64, Stack<f64, MultiPass<U2, f64>, Constant<U1, f64>>, Svf<f64, f64, HighpassMode<f64>>>,
> {
    super::prelude::highpass_q::<f64, f64>(q)
}

/// Bandpass filter.
/// - Input 0: audio
/// - Input 1: center frequency (Hz)
/// - Input 2: Q
/// - Output 0: filtered audio
#[inline]
pub fn bandpass() -> An<Svf<f64, f64, BandpassMode<f64>>> {
    super::prelude::bandpass()
}

/// Bandpass filter centered at `f` Hz with Q value `q`.
/// - Input 0: audio
/// - Output 0: filtered audio
#[inline]
pub fn bandpass_hz(f: f64, q: f64) -> An<FixedSvf<f64, f64, BandpassMode<f64>>> {
    super::prelude::bandpass_hz::<f64, f64>(f, q)
}

/// Bandpass filter with Q value `q`.
/// - Input 0: audio
/// - Input 1: center frequency (Hz)
/// - Output 0: filtered audio
#[inline]
pub fn bandpass_q(
    q: f64,
) -> An<
    Pipe<f64, Stack<f64, MultiPass<U2, f64>, Constant<U1, f64>>, Svf<f64, f64, BandpassMode<f64>>>,
> {
    super::prelude::bandpass_q::<f64, f64>(q)
}

/// Notch filter.
/// - Input 0: audio
/// - Input 1: center frequency (Hz)
/// - Input 2: Q
/// - Output 0: filtered audio
#[inline]
pub fn notch() -> An<Svf<f64, f64, NotchMode<f64>>> {
    super::prelude::notch()
}

/// Notch filter centered at `f` Hz with Q value `q`.
/// - Input 0: audio
/// - Output 0: filtered audio
#[inline]
pub fn notch_hz(f: f64, q: f64) -> An<FixedSvf<f64, f64, NotchMode<f64>>> {
    super::prelude::notch_hz::<f64, f64>(f, q)
}

/// Notch filter with Q value `q`.
/// - Input 0: audio
/// - Input 1: center frequency (Hz)
/// - Output 0: filtered audio
#[inline]
pub fn notch_q(
    q: f64,
) -> An<Pipe<f64, Stack<f64, MultiPass<U2, f64>, Constant<U1, f64>>, Svf<f64, f64, NotchMode<f64>>>>
{
    super::prelude::notch_q::<f64, f64>(q)
}

/// Peaking filter.
/// - Input 0: audio
/// - Input 1: center frequency (Hz)
/// - Input 2: Q
/// - Output 0: filtered audio
#[inline]
pub fn peak() -> An<Svf<f64, f64, PeakMode<f64>>> {
    super::prelude::peak()
}

/// Peaking filter centered at `f` Hz with Q value `q`.
/// - Input 0: audio
/// - Output 0: filtered audio
#[inline]
pub fn peak_hz(f: f64, q: f64) -> An<FixedSvf<f64, f64, PeakMode<f64>>> {
    super::prelude::peak_hz::<f64, f64>(f, q)
}

/// Peaking filter with Q value `q`.
/// - Input 0: audio
/// - Input 1: center frequency (Hz)
/// - Output 0: filtered audio
#[inline]
pub fn peak_q(
    q: f64,
) -> An<Pipe<f64, Stack<f64, MultiPass<U2, f64>, Constant<U1, f64>>, Svf<f64, f64, PeakMode<f64>>>>
{
    super::prelude::peak_q::<f64, f64>(q)
}

/// Allpass filter.
/// - Input 0: audio
/// - Input 1: center frequency (Hz)
/// - Input 2: Q
/// - Output 0: filtered audio
#[inline]
pub fn allpass() -> An<Svf<f64, f64, AllpassMode<f64>>> {
    super::prelude::allpass()
}

/// Allpass filter centered at `f` Hz with Q value `q`.
/// - Input 0: audio
/// - Output 0: filtered audio
#[inline]
pub fn allpass_hz(f: f64, q: f64) -> An<FixedSvf<f64, f64, AllpassMode<f64>>> {
    super::prelude::allpass_hz::<f64, f64>(f, q)
}

/// Allpass filter with Q value `q`.
/// - Input 0: audio
/// - Input 1: center frequency (Hz)
/// - Output 0: filtered audio
#[inline]
pub fn allpass_q(
    q: f64,
) -> An<Pipe<f64, Stack<f64, MultiPass<U2, f64>, Constant<U1, f64>>, Svf<f64, f64, AllpassMode<f64>>>>
{
    super::prelude::allpass_q::<f64, f64>(q)
}

/// Bell filter with adjustable gain.
/// - Input 0: audio
/// - Input 1: center frequency (Hz)
/// - Input 2: Q
/// - Input 3: amplitude gain
/// - Output 0: filtered audio
#[inline]
pub fn bell() -> An<Svf<f64, f64, BellMode<f64>>> {
    super::prelude::bell()
}

/// Bell filter centered at `f` Hz with Q value `q` and amplitude gain `gain`.
/// - Input 0: audio
/// - Output 0: filtered audio
#[inline]
pub fn bell_hz(f: f64, q: f64, gain: f64) -> An<FixedSvf<f64, f64, BellMode<f64>>> {
    super::prelude::bell_hz::<f64, f64>(f, q, gain)
}

/// Bell filter with Q value `q` and amplitude gain `gain`.
/// - Input 0: audio
/// - Input 1: center frequency
/// - Output 0: filtered audio
#[inline]
pub fn bell_q(
    q: f64,
    gain: f64,
) -> An<Pipe<f64, Stack<f64, MultiPass<U2, f64>, Constant<U2, f64>>, Svf<f64, f64, BellMode<f64>>>>
{
    super::prelude::bell_q::<f64, f64>(q, gain)
}

/// Low shelf filter with adjustable gain.
/// - Input 0: audio
/// - Input 1: center frequency (Hz)
/// - Input 2: Q
/// - Input 3: amplitude gain
/// - Output 0: filtered audio
#[inline]
pub fn lowshelf() -> An<Svf<f64, f64, LowshelfMode<f64>>> {
    super::prelude::lowshelf()
}

/// Low shelf filter centered at `f` Hz with Q value `q` and amplitude gain `gain`.
/// - Input 0: audio
/// - Output 0: filtered audio
#[inline]
pub fn lowshelf_hz(f: f64, q: f64, gain: f64) -> An<FixedSvf<f64, f64, LowshelfMode<f64>>> {
    super::prelude::lowshelf_hz::<f64, f64>(f, q, gain)
}

/// Low shelf filter with Q value `q` and amplitude gain `gain`.
/// - Input 0: audio
/// - Input 1: cutoff frequency
/// - Output 0: filtered audio
#[inline]
pub fn lowshelf_q(
    q: f64,
    gain: f64,
) -> An<
    Pipe<f64, Stack<f64, MultiPass<U2, f64>, Constant<U2, f64>>, Svf<f64, f64, LowshelfMode<f64>>>,
> {
    super::prelude::lowshelf_q::<f64, f64>(q, gain)
}

/// High shelf filter with adjustable gain.
/// - Input 0: audio
/// - Input 1: center frequency (Hz)
/// - Input 2: Q
/// - Input 3: amplitude gain
/// - Output 0: filtered audio
#[inline]
pub fn highshelf() -> An<Svf<f64, f64, HighshelfMode<f64>>> {
    super::prelude::highshelf()
}

/// High shelf filter centered at `cutoff` Hz with Q value `q` and amplitude gain `gain`.
/// - Input 0: audio
/// - Output 0: filtered audio
#[inline]
pub fn highshelf_hz(f: f64, q: f64, gain: f64) -> An<FixedSvf<f64, f64, HighshelfMode<f64>>> {
    super::prelude::highshelf_hz::<f64, f64>(f, q, gain)
}

/// High shelf filter with Q value `q` and amplitude gain `gain`.
/// - Input 0: audio
/// - Input 1: cutoff frequency
/// - Output 0: filtered audio
#[inline]
pub fn highshelf_q(
    q: f64,
    gain: f64,
) -> An<
    Pipe<f64, Stack<f64, MultiPass<U2, f64>, Constant<U2, f64>>, Svf<f64, f64, HighshelfMode<f64>>>,
> {
    super::prelude::highshelf_q::<f64, f64>(q, gain)
}

/// Pulse wave oscillator.
/// - Input 0: frequency in Hz
/// - Input 1: pulse duty cycle in 0...1
/// - Output 0: pulse wave
#[inline]
pub fn pulse() -> An<super::prelude::PulseWave<f64>> {
    super::prelude::pulse()
}
