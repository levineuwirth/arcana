//! Flexible Waterbender — `{3}{U}` 2/5 Creature — Human Warrior Ally. Vigilance.
//! "Waterbend {3}: This creature has base power and toughness 5/2 until end of
//! turn."
//!
//! Vigilance is a base keyword. Waterbend is not in the usable keyword surface,
//! but the ability is a plain `{3}`-cost activated ability whose effect sets
//! base P/T to 5/2 until end of turn. The waterbend cost's "tap artifacts and
//! creatures to help pay {1} each" (convoke-style) rider is a GAP — only the
//! {3} mana cost is modeled.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Flexible Waterbender");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let ally = reg.interner_mut().intern("Ally");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);
    subtypes.0.insert(ally);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "Waterbend {3}: Flexible Waterbender has base power and toughness 5/2 until end of turn.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{3}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: set_base_pt,
        }),
    )
}

fn set_base_pt(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::SetBasePT {
        target: ctx.source,
        power: 5,
        toughness: 2,
        duration: Duration::EndOfTurn,
    }]
}
