// FIXME: Make me pass! Diff budget: 25 lines.



enum Duration {
    MilliSeconds(u64),
    Seconds(u32),
    Minutes(u16)
}

<<<<<<< HEAD

// What traits does `Duration` need to implement?
impl PartialEq for Duration {
	fn eq(&self, other:&Duration) -> bool {
		let anum = match *self {
			Duration::MilliSeconds(time)=> time as u64,
			Duration::Seconds(ntime)=> ntime as u64*1000 as u64,
			Duration::Minutes(stime)=> stime as u64*60000 as u64,
		};
		let bnum = match *other {
			Duration::MilliSeconds(time)=> time as u64,
			Duration::Seconds(ntime)=> ntime as u64 * 1000 as u64,
			Duration::Minutes(stime)=> stime as u64 *60000 as u64,
		};
		anum == bnum	
	}
}
trait Eq: PartialEq<Self> {}

=======
>>>>>>> skeleton/lab2
#[test]
fn traits() {
    assert_eq!(Seconds(120), Minutes(2));
    assert_eq!(Seconds(420), Minutes(7));
    assert_eq!(MilliSeconds(420000), Minutes(7));
    assert_eq!(MilliSeconds(43000), Seconds(43));
}
