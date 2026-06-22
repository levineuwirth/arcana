//! Varchild, Betrayer of Kjeldor — `{2}{R}` 3/3 Legendary Human Knight.
//!
//! * "Whenever Varchild deals combat damage to a player, that player creates
//!   that many 1/1 red Survivor creature tokens." — a `DamageDealt` (combat,
//!   to a player) trigger; the count is the combat damage dealt
//!   (`trig.damage_amount()`), the controller is the damaged player
//!   (`trig.damaged_player()`).
//! * "Survivors your opponents control can't block, and they can't attack you
//!   or planeswalkers you control." — GAP: a continuous static restriction on
//!   other permanents; not a triggered/activated ability.
//! * "When Varchild leaves the battlefield, gain control of all Survivors." —
//!   a `SelfLeavesBattlefield` trigger; we enumerate every Survivor on the
//!   battlefield and emit a `ChangeControl` per id (ForEach does not
//!   substitute per-id for ChangeControl, so a Sequence over the ids is used).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::TargetFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Varchild, Betrayer of Kjeldor");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let _survivor = reg.interner_mut().intern("Survivor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: "Survivors your opponents control can't block, and they can't attack
    // you or planeswalkers you control." — continuous static restriction on
    // other permanents; not expressible as a triggered/activated ability.

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    // No self-source DamageDealt predicate exists; scope to this
                    // card by name so only Varchild's combat damage fires.
                    source_filter: arcana_core::targets::ObjectFilter {
                        name: Some(name),
                        ..arcana_core::targets::ObjectFilter::default()
                    },
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: make_survivor_tokens,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfLeavesBattlefield,
                intervening_if: None,
                effect: gain_control_of_survivors,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// "that player creates that many 1/1 red Survivor creature tokens."
fn make_survivor_tokens(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let n = trig.damage_amount().unwrap_or(0);
    if n == 0 {
        return Vec::new();
    }
    let Some(p) = trig.damaged_player() else {
        return Vec::new();
    };
    let survivor = reg.interner().lookup("Survivor").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(survivor);
    let token = TokenDefinition {
        name: survivor,
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    (0..n)
        .map(|_| Effect::CreateToken {
            controller: p,
            token: token.clone(),
        })
        .collect()
}

/// "gain control of all Survivors" — one ChangeControl per Survivor on the
/// battlefield (ChangeControl is not retargeted by ForEach, so use Sequence).
fn gain_control_of_survivors(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = script::subtype_filter(reg, "Survivor");
    let ids = script::ids_matching(state, &filter, trig.controller);
    let effects: Vec<Effect> = ids
        .into_iter()
        .map(|id| Effect::ChangeControl {
            target: id,
            new_controller: trig.controller,
        })
        .collect();
    if effects.is_empty() {
        return Vec::new();
    }
    vec![Effect::Sequence(effects)]
}
