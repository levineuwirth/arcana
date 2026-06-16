//! Quicksilver Behemoth — `{6}{U}` 4/5 Beast.
//!
//! Scryfall keyword "Affinity" (for artifacts) is a cost-reduction
//! static, not a usable `KeywordAbility` variant. keywords: vec![].
//! GAP (keyword/static): "Affinity for artifacts".
//!
//! "When this creature attacks or blocks, return it to its owner's
//! hand at end of combat." — there is no combined "attacks or blocks"
//! TriggerCondition, so this is decomposed into two triggers
//! (SelfAttacks, SelfBlocks). The "return at end of combat" delayed
//! action has no end-of-combat delayed-trigger variant in the catalog,
//! so each effect is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Quicksilver Behemoth");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: return_at_end_of_combat,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfBlocks,
                intervening_if: None,
                effect: return_at_end_of_combat,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// "Return it to its owner's hand at end of combat."
fn return_at_end_of_combat(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: no end-of-combat delayed-trigger variant exists in the
    // effect/DelayedWhen catalog (only NextEndStep / NextUpkeep /
    // ThisDies), so the delayed "return to owner's hand at end of
    // combat" cannot be scheduled. Effect omitted.
    Vec::new()
}
