//! Thraben Doomsayer — `{1}{W}{W}` 2/2 Human Cleric.
//! "{T}: Create a 1/1 white Human creature token.
//!  Fateful hour — As long as you have 5 or less life, other creatures
//!  you control get +2/+2."
//!
//! The tap-to-make-a-token activated ability is wired. The Fateful-hour
//! life-gated static anthem is a continuous static, not a triggered or
//! activated ability — GAP'd.

use arcana_core::effects::Effect;
use arcana_core::objects::Characteristics;
use arcana_core::mana::ManaCost;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::effects::TokenDefinition;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

// GAP: static — "Fateful hour — As long as you have 5 or less life, other
// creatures you control get +2/+2" is a life-gated continuous anthem (no
// trigger word, no activation cost); not expressible as a triggered/activated
// ability.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thraben Doomsayer");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Create a 1/1 white Human creature token.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_human_token,
            }),
    )
}

fn make_human_token(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let human = reg.interner().lookup("Human").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: human,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
