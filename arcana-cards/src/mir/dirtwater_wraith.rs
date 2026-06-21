//! Dirtwater Wraith — `{3}{B}` 1/3 Wraith.
//! Swampwalk; `{B}: This creature gets +1/+0 until end of turn.`

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
    let name = reg.interner_mut().intern("Dirtwater Wraith");
    let wraith = reg.interner_mut().intern("Wraith");
    let swamp = reg.interner_mut().intern("Swamp");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wraith);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Landwalk(swamp)],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{B}: This creature gets +1/+0 until end of turn.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{B}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: pump_self,
        }),
    )
}

fn pump_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Pump {
        target: ctx.source,
        power: 1,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
