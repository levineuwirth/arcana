//! Casey Jones, Vigilante — `{1}{R}{R}` 4/3 red Legendary Creature — Human Berserker.
//! "When Casey Jones enters, draw three cards. At the beginning of your next upkeep,
//! discard three cards at random."

use arcana_core::effects::{DelayedWhen, DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Casey Jones, Vigilante");
    let human = reg.interner_mut().intern("Human");
    let berserker = reg.interner_mut().intern("Berserker");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(berserker);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_draw_then_discard,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_draw_then_discard(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // Draw 3 now; schedule the random discard-3 as a delayed effect.
    // GAP (narrow): printed timing is "YOUR next upkeep" —
    // DelayedWhen::NextUpkeep fires at the next upkeep that begins,
    // whoever's turn it is (may be an opponent's upkeep first).
    vec![
        Effect::DrawCards { player: trig.controller, count: 3 },
        Effect::ScheduleDelayedEffect {
            source: trig.source,
            controller: trig.controller,
            when: DelayedWhen::NextUpkeep,
            effect: delayed_random_discard,
        },
    ]
}

/// "At the beginning of your next upkeep, discard three cards at
/// random." Runs even if Casey Jones has left play.
fn delayed_random_discard(
    _state: &GameState,
    pt: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Discard {
        player: pt.controller,
        count: 3,
        choice: DiscardChoice::Random,
    }]
}
