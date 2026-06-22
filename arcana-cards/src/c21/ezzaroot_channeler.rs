//! Ezzaroot Channeler — `{5}{G}` 4/6 Treefolk Druid with Reach.
//!
//! Oracle:
//! * Reach.
//! * Creature spells you cast cost {X} less to cast, where X is the amount of
//!   life you gained this turn. (static cost reduction — GAP)
//! * {T}: You gain 2 life.

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
    let name = reg.interner_mut().intern("Ezzaroot Channeler");
    let treefolk = reg.interner_mut().intern("Treefolk");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(treefolk);
    subtypes.0.insert(druid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };

    // GAP: static cost reduction "Creature spells you cast cost {X} less, where
    // X is the life you gained this turn" — no cost-reduction Effect/static in
    // the demonstrated API.
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: You gain 2 life.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: gain_two_life,
            }),
    )
}

fn gain_two_life(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::GainLife { player: ctx.controller, amount: 2 }]
}
