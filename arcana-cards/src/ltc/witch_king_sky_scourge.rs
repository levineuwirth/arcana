//! Witch-king, Sky Scourge — `{5}{B}{R}` 5/5 Legendary Wraith Noble (B/R).
//! Flying, Undying. Whenever you attack with one or more Wraiths, exile the
//! top X cards of your library (X = their total power); you may play them
//! this turn (dynamic X over the attacking batch is not computable from the
//! available script helpers → effect GAP'd).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Witch-king, Sky Scourge");
    let wraith = reg.interner_mut().intern("Wraith");
    let noble = reg.interner_mut().intern("Noble");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wraith);
    subtypes.0.insert(noble);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Undying],
        ..Default::default()
    };

    let wraith_filter = script::subtype_filter(reg, "Wraith")
        .controlled_by(ControllerConstraint::You);

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: wraith_filter,
                },
                intervening_if: None,
                effect: wraith_attack_impulse,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn wraith_attack_impulse(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: X = total power of the attacking Wraiths (a per-attack batch
    // sum) is not computable from the available script helpers, so the
    // ImpulseExile count is unknown; a fixed count would be wrong.
    Vec::new()
}
