// 1. Thank you for volunteering to document our pumpkin library! Let's start by grabbing the first
// paragraph from https://en.wikipedia.org/wiki/Pumpkin and pasting it as our module-level
// documentation. Hint: Use inner-documentation comments.
//
// Once you've got the documentation here, run `cargo doc --no-deps --open` and take a look!

//! A pumpkin is a cultivar of winter squash that is round with smooth, slightly ribbed skin,
//! and is most often deep yellow to orange in coloration. The thick shell contains the seeds and pulp. 
//! The name is most commonly used for cultivars of Cucurbita pepo, but some cultivars of Cucurbita maxima, 
//! C. argyrosperma, and C. moschata with similar appearance are also sometimes called "pumpkins".
//! ![some alt text](https://upload.wikimedia.org/wikipedia/commons/thumb/5/5c/FrenchMarketPumpkinsB.jpg/700px-FrenchMarketPumpkinsB.jpg)

/// Big orange thing
/// 
/// # Receipes
/// 
/// Coming soon...
pub struct Pumpkin {
    /// percentage
    pub roundness: f32,
    /// number from 8 to 27
    pub orangeness: i32,
}

// 4. Document the "smash" method. Explain that if you smash the pumpkin, it will be gone. Then it
// can't be used for pie. :'-(

impl Pumpkin {
    /// if you smash the pumpkin, it will be gone
    /// 
    /// can't be used for pie. :'-(
    pub fn smash(self) {}
}

// 5. Document that BURNT_ORANGE is for the "orangeness" field in the Pumpkin struct.
// - Link to the Pumpkin struct in your description

/// "orangeness" field in the Pumpkin struct
/// 
/// [Pumpkin]
pub const BURNT_ORANGE: i32 = 13;

// Challenge: Find the option to pass to `cargo doc` so that documentation for this private item
// gets generated as well.  Hint: `cargo doc -h` will show you all the relevant options.
// --document-private-items

/// For internal use only. In fact, this documentation is so private that it won't be generated.
/// At least not by default. But if you pass the correct option in, it will magically appear!
#[allow(dead_code)] // to silence the warning
enum PrivateEnum {
    /// For Halloween. To be lit by candlelight.
    JackOLantern,
    /// For dessert during North American winter holidays.
    PumpkinPie,
}
