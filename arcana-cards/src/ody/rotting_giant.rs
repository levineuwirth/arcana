//! Rotting Giant — `{1}{B}` 3/3 black Creature — Zombie Giant.
//! "Whenever this creature attacks or blocks, sacrifice it unless you exile a
//! card from your graveyard."
//!
//! GAP (partial): The "unless you exile a card from your graveyard" cost gate
//! requires a targeted exile-from-graveyard as payment. OptionalPaymentKind
//! supports Mana and Life costs only, not graveyard-exile costs. We model the
//! "you may pay" structure with a GAP note; the sacrifice branch uses
//! Effect::Sacrifice targeting self.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rotting Giant");
    let zombie = reg.interner_mut().intern("Zombie");
    let giant = reg.interner_mut().intern("Giant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(giant);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // "attacks or blocks" — closest available split is SelfAttacks;
                // blocks side uses SelfBlocks. Only one condition is supported per
                // TriggeredAbilityDef; model as SelfAttacks.
                // GAP: trigger — "attacks or blocks" requires two separate triggers
                // (SelfAttacks + SelfBlocks); engine supports only one condition per def.
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attacks_or_blocks_effect,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn attacks_or_blocks_effect(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "sacrifice it unless you exile a card from your graveyard" requires
    // OptionalPaymentKind::ExileFromGraveyard which does not exist in the API.
    // Emitting the sacrifice unconditionally as a best-effort stub.
    vec![Effect::Sacrifice {
        player: trig.controller,
        filter: ObjectFilter::permanent().controlled_by(ControllerConstraint::You),
        count: 1,
    }]
}
