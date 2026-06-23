//! Fear of Missing Out — `{1}{R}` 2/3 red Enchantment Creature — Nightmare.
//!
//! Oracle:
//! * "When this creature enters, discard a card, then draw a card." —
//!   ETB trigger (Discard + DrawCards).
//! * "Delirium — Whenever this creature attacks for the first time each
//!   turn, if there are four or more card types among cards in your
//!   graveyard, untap target creature. After this phase, there is an
//!   additional combat phase." — once-per-turn attack trigger; the
//!   delirium intervening-if (four-or-more card types in graveyard) is
//!   not in the documented `conditions::` surface so it is GAP'd, but
//!   the effect (untap target creature + an additional combat phase) is
//!   wired faithfully.
//!
//! Delirium is not a `KeywordAbility` variant — it is the ability-word
//! gate above, not a keyword; `keywords` stays empty.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fear of Missing Out");
    let nightmare = reg.interner_mut().intern("Nightmare");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nightmare);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_discard_then_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // GAP: delirium intervening-if "if there are four or more card
            // types among cards in your graveyard" is not in the documented
            // conditions:: surface — fired unconditionally (None) per spec.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_untap_extra_combat,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::OncePerTurn,
                target_requirements: vec![TargetRequirement::target_creature()],
            }),
    )
}

fn etb_discard_then_draw(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::Discard {
            player: trig.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        },
        Effect::DrawCards { player: trig.controller, count: 1 },
    ]
}

fn attack_untap_extra_combat(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![
        Effect::Untap { target: *id },
        Effect::AdditionalCombatPhase,
    ]
}
