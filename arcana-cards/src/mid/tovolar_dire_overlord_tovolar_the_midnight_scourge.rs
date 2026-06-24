//! Tovolar, Dire Overlord // Tovolar, the Midnight Scourge — `{1}{R}{G}` Legendary Human Werewolf 3/3.
//! Front: Whenever a Wolf or Werewolf you control deals combat damage to a player, draw a card.
//! At the beginning of your upkeep, if you control three or more Wolves and/or Werewolves, it becomes
//! night. Then transform any number of Human Werewolves you control.
//! Daybound (GAP: Daybound/Nightbound day/night cycle not modeled; transform wired via upkeep trigger).
//! Back (Tovolar, the Midnight Scourge): Whenever a Wolf or Werewolf you control deals combat damage
//! to a player, draw a card. {X}{R}{G}: Target Wolf or Werewolf you control gets +X/+0 and gains
//! trample until end of turn. Nightbound (GAP: Nightbound not modeled).
//! "if you control three or more Wolves and/or Werewolves" intervening-if wired on the upkeep
//! transform trigger via a manual with_subtypes_any(["Wolf","Werewolf"]) filter + you_control_at_least.
//! The "Whenever a Wolf or Werewolf you control deals combat damage to a player, draw a card"
//! trigger (present on BOTH faces) is WIRED via TriggerCondition::DamageDealt with a
//! subtype-filtered, controller=You source_filter; left ungated so it fires on both faces.
//! GAP: "transform any number of Human Werewolves you control" — mass transform of others not expressible.
//! GAP: Back-face activated ability ({X}{R}{G}: pump target Wolf/Werewolf) not modeled — there is
//!      no generic-{X} mana cost on activated abilities (only loyalty-X fans out; a normal
//!      activated ability's {X} silently resolves to 0). Missing primitive: an ActivationCost
//!      generic-X fan-out feeding the chosen X into ActivationContext.x_value.

use arcana_core::conditions;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::objects::ObjectId;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{
    CardId, ColorSet, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::state::GameState;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tovolar, Dire Overlord");
    let human_sub = reg.interner_mut().intern("Human");
    let werewolf_sub = reg.interner_mut().intern("Werewolf");
    let wolf_sub = reg.interner_mut().intern("Wolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(werewolf_sub);

    // Source filter for "a Wolf or Werewolf you control deals combat damage".
    let wolf_or_werewolf_you_control = ObjectFilter::creature()
        .with_subtypes_any(vec![wolf_sub, werewolf_sub])
        .controlled_by(ControllerConstraint::You);

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
            // Trigger 1: "Whenever a Wolf or Werewolf you control deals combat damage to a
            // player, draw a card." Present on BOTH faces — left ungated so it fires on each.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: wolf_or_werewolf_you_control,
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: draw_a_card,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Trigger 2: upkeep transform (front face only — "it becomes night, then
            // transform" when you control three+ Wolves/Werewolves).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: Some(iif_three_wolves_or_werewolves),
                effect: upkeep_transform,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Combat-damage draw (1) is on both faces (ungated); the upkeep transform (2)
            // is front-face only.
            .with_trigger_face_gate(2, 0),
    )
}

fn draw_a_card(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::DrawCards {
        player: trig.controller,
        count: 1,
    }]
}

/// "if you control three or more Wolves and/or Werewolves" — OR of two subtypes
/// via a manually-built with_subtypes_any filter (empty Vec ⇒ matches nothing ⇒
/// false, which is correct if neither subtype name interns).
fn iif_three_wolves_or_werewolves(
    state: &GameState,
    _source: ObjectId,
    you: PlayerId,
    reg: &CardRegistry,
) -> bool {
    let subs: Vec<_> = ["Wolf", "Werewolf"]
        .iter()
        .filter_map(|n| reg.interner().lookup(n))
        .collect();
    conditions::you_control_at_least(state, you, &ObjectFilter::new().with_subtypes_any(subs), 3)
}

fn upkeep_transform(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "it becomes night" — day/night cycle not modeled.
    // GAP: "transform any number of Human Werewolves you control" — mass transform not expressible.
    // Emitting the transform of the source as a best-effort approximation.
    vec![Effect::Transform { target: trig.source }]
}
