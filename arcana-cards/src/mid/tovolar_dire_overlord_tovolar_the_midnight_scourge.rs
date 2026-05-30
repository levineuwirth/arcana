//! Tovolar, Dire Overlord // Tovolar, the Midnight Scourge — `{1}{R}{G}` Legendary Human Werewolf 3/3.
//! Front: Whenever a Wolf or Werewolf you control deals combat damage to a player, draw a card.
//! At the beginning of your upkeep, if you control three or more Wolves and/or Werewolves, it becomes
//! night. Then transform any number of Human Werewolves you control.
//! Daybound (GAP: Daybound/Nightbound day/night cycle not modeled; transform wired via upkeep trigger).
//! Back (Tovolar, the Midnight Scourge): Whenever a Wolf or Werewolf you control deals combat damage
//! to a player, draw a card. {X}{R}{G}: Target Wolf or Werewolf you control gets +X/+0 and gains
//! trample until end of turn. Nightbound (GAP: Nightbound not modeled).
//! GAP: "if you control three or more Wolves and/or Werewolves" intervening-if condition not modeled.
//! GAP: "transform any number of Human Werewolves you control" — mass transform of others not expressible.
//! GAP: Back-face activated ability ({X}{R}{G}: pump target Wolf/Werewolf) not modeled (face-gated activation with X cost not in engine).
//! GAP: back-face-only triggered ability not modeled for the combat-damage draw trigger on back face.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;
use arcana_core::targets::ControllerConstraint;
use arcana_core::state::GameState;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tovolar, Dire Overlord");
    let human_sub = reg.interner_mut().intern("Human");
    let werewolf_sub = reg.interner_mut().intern("Werewolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(werewolf_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Tovolar, the Midnight Scourge");
    let back_werewolf_sub = reg.interner_mut().intern("Werewolf");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_werewolf_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red() | ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            power: Some(PtValue::Fixed(6)),
            toughness: Some(PtValue::Fixed(6)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Trigger 1: combat-damage draw (front face — Wolf or Werewolf deals combat damage)
            // GAP: TriggerCondition::CombatDamageDealt with subtype filter not available; using ZoneChange
            // as placeholder. The actual trigger fires when a Wolf/Werewolf deals combat damage to a player.
            // GAP: "whenever a Wolf or Werewolf you control deals combat damage to a player" — no
            // CombatDamageDealt trigger condition; omitting this triggered ability.
            // Trigger 2: upkeep transform trigger (front face — transform when three+ Wolves/Werewolves)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: upkeep_transform,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
    )
}

fn upkeep_transform(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: intervening-if "if you control three or more Wolves and/or Werewolves" not modeled.
    // GAP: "it becomes night" — day/night cycle not modeled.
    // GAP: "transform any number of Human Werewolves you control" — mass transform not expressible.
    // Emitting the transform of the source as a best-effort approximation.
    vec![Effect::Transform { target: trig.source }]
}
