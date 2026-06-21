//! Simian Sling — `{R}` 1/1 red Artifact Creature — Equipment Monkey.
//!
//! Oracle:
//! * "Equipped creature gets +1/+1." — a static buff granted to the
//!   equipped creature; no expressible equipped-creature static-pump
//!   primitive. GAP'd.
//! * "Whenever this creature or equipped creature becomes blocked, it
//!   deals 1 damage to defending player." — modeled via `SelfBecomesBlocked`
//!   for the THIS-creature half: when it becomes blocked, deal 1 damage to
//!   the defending player (the blocker's controller, found via
//!   `other_combatant()`). The "equipped creature becomes blocked" half is
//!   not separately expressible (no trigger keyed on the equipped creature)
//!   — GAP'd within this ability.
//! * Reconfigure {2} — the `Reconfigure` keyword has no `KeywordAbility`
//!   variant and the reconfigure attach/unattach activated mechanic is not
//!   expressible. GAP'd.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
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
    let name = reg.interner_mut().intern("Simian Sling");
    let equipment = reg.interner_mut().intern("Equipment");
    let monkey = reg.interner_mut().intern("Monkey");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(equipment);
    subtypes.0.insert(monkey);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: keyword — Reconfigure has no KeywordAbility variant.
        keywords: vec![],
        ..Default::default()
    };

    // GAP: static — "Equipped creature gets +1/+1" (no equipped-creature
    // static-pump primitive).
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfBecomesBlocked,
                intervening_if: None,
                effect: damage_defending_player,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn damage_defending_player(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // The defending player is the controller of the blocking creature.
    let Some(blocker) = trig.other_combatant() else {
        return Vec::new();
    };
    let Some(player) = state.objects.get(blocker).map(|o| o.controller) else {
        return Vec::new();
    };
    vec![Effect::DealDamage {
        source: trig.source,
        target: DamageTarget::Player(player),
        amount: 1,
    }]
}
