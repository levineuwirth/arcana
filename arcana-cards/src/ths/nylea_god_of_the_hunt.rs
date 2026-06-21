//! Nylea, God of the Hunt — `{3}{G}` 6/6 Legendary Enchantment Creature — God.
//!
//! Indestructible.
//! As long as your devotion to green is less than five, Nylea isn't a creature.
//! Other creatures you control have trample.
//! {3}{G}: Target creature gets +2/+2 until end of turn.
//!
//! Indestructible is a base keyword. The two statics ("isn't a creature unless
//! devotion …" and "other creatures you control have trample") are pure
//! continuous statics and are GAP'd. The activated +2/+2 pump is implemented.

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nylea, God of the Hunt");
    let god = reg.interner_mut().intern("God");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(god);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet::new().with(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Indestructible],
        ..Default::default()
    };

    // GAP: "As long as your devotion to green is less than five, Nylea isn't a
    // creature" — conditional type-removing static, no primitive.
    // GAP: "Other creatures you control have trample" — static keyword anthem,
    // no primitive in this surface.

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{3}{G}: Target creature gets +2/+2 until end of turn.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{3}{G}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement::target_creature()],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: pump_target,
        }),
    )
}

fn pump_target(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::Pump {
        target: *id,
        power: 2,
        toughness: 2,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
