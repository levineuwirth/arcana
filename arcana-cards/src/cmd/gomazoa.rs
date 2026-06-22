//! Gomazoa — `{2}{U}` 0/3 Jellyfish.
//! Defender, flying; "{T}: Put this creature and each creature it's
//! blocking on top of their owners' libraries, then those players shuffle."
//!
//! The activated ability puts THIS creature on top of its owner's
//! library (the shuffle is automatic for a library-bound move). The
//! "and each creature it's blocking" clause is GAP'd: there is no
//! activated-context accessor for the set of creatures this object is
//! currently blocking, and no library-put effect over a computed set.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gomazoa");
    let jellyfish = reg.interner_mut().intern("Jellyfish");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(jellyfish);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Defender, KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}: Put this creature and each creature it's blocking on top of their owners' libraries, then those players shuffle.".into(),
            cost: ActivationCost::tap_only(),
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: put_self_on_library,
        }),
    )
}

fn put_self_on_library(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "and each creature it's blocking" — no accessor for the
    //      set of creatures this object is blocking at activation time.
    vec![Effect::PutOnTopOfLibrary { target: ctx.source }]
}
