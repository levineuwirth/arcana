//! Mendicant Core, Guidelight — `{W}{U}` */3 Legendary Artifact Creature — Robot.
//!
//! * Mendicant Core's power is equal to the number of artifacts you control.
//!   (GAP — CDA; no characteristic-defining-ability primitive)
//! * Start your engines! / speed mechanic. (GAP — not modeled)
//! * Max speed — Whenever you cast an artifact spell, you may pay {1}. If you
//!   do, copy it. (GAP)
//!
//! Power is recorded as `Star` to capture the `*/3` bones; the artifact-count
//! CDA itself has no primitive and is GAP'd. The speed mechanic ("Start your
//! engines!", Max speed) is unmodeled. The Max-speed copy trigger is GAP'd:
//! it is gated on having max speed (unexpressible) and `CopySpell` needs the
//! just-cast spell's id, which a `SpellCast` trigger exposes no accessor for.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mendicant Core, Guidelight");
    let robot = reg.interner_mut().intern("Robot");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(robot);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::white(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        // GAP: CDA — power equals the number of artifacts you control; no CDA
        //       primitive. Star records the printed `*`.
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: "Start your engines!" and the speed mechanic are not modeled.
    // GAP: "Max speed — Whenever you cast an artifact spell, you may pay {1}.
    //       If you do, copy it." — gated on max speed (unexpressible) and
    //       CopySpell has no accessor for the just-cast spell's id from a
    //       SpellCast trigger.

    reg.register(CardDefinition::new(name, chars))
}
