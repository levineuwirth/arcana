//! Casey Jones, Vigilante — `{1}{R}{R}` 4/3 red Legendary Creature — Human Berserker.
//! "When Casey Jones enters, draw three cards. At the beginning of your next upkeep,
//! discard three cards at random."

use arcana_core::effects::{DelayedAction, DelayedWhen, DiscardChoice, Effect};
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
    // Draw 3, then schedule a discard-3 at next upkeep via a separate trigger.
    // GAP: "at beginning of your next upkeep, discard three" — DelayedAction only supports
    // Sacrifice/Exile/ReturnToHand/ReturnFromExileToBattlefield, not Discard.
    // Emitting the draw; the delayed discard is approximated inline.
    vec![
        Effect::DrawCards { player: trig.controller, count: 3 },
        Effect::Discard { player: trig.controller, count: 3, choice: DiscardChoice::Random },
    ]
}
