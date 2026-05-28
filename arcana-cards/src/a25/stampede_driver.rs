//! Stampede Driver — `{G}` 1/1 green Human Spellshaper.
//! "{1}{G}, {T}, Discard a card: Creatures you control get +1/+1 and gain trample until end of turn."
//!
//! GAP: The discard-a-card cost cannot be expressed via ActivationCost (no discard_other field;
//! discard_self is for discarding the source card itself). The best approximation uses only
//! mana + tap; the discard cost is dropped.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::objects::NULL_OBJECT_ID;
use arcana_core::effects::KeywordAbility;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Stampede Driver");
    let human = reg.interner_mut().intern("Human");
    let spellshaper = reg.interner_mut().intern("Spellshaper");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(spellshaper);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{G}, {T}, Discard a card: Creatures you control get +1/+1 and gain trample until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{G}").unwrap(),
                    tap: true,
                    // GAP: discard a card cost not expressible (no discard_other field in ActivationCost)
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_creatures_trample,
            }),
    )
}

fn pump_creatures_trample(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        ctx.controller,
    );
    ids.into_iter().flat_map(|id| {
        vec![
            Effect::Pump {
                target: id,
                power: 1,
                toughness: 1,
                duration: Duration::EndOfTurn,
                keywords: vec![KeywordAbility::Trample],
            },
        ]
    }).collect()
}
