//! Krydle of Baldur's Gate — `{U}{B}` 1/3 Legendary Creature —
//! Human Elf Rogue.
//!
//! Oracle:
//! * "Whenever Krydle deals combat damage to a player, that player loses 1
//!   life and mills a card, then you gain 1 life and scry 1." — a
//!   combat-damage-to-a-player trigger restricted to this creature (source
//!   name filter); the damaged player loses 1 life and mills 1, then you gain
//!   1 life and scry 1.
//! * "Whenever you attack, you may pay {2}. If you do, target creature can't
//!   be blocked this turn." — an attack trigger that targets a creature and
//!   offers a {2} payment to make it unblockable. "Whenever you attack" is
//!   approximated by "whenever a creature you control attacks" (no
//!   declare-attackers-step trigger exists in the demonstrated surface).
//! (The Scryfall "Mill"/"Scry" keywords are reminder text, not KeywordAbility.)

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Krydle of Baldur's Gate");
    let human = reg.interner_mut().intern("Human");
    let elf = reg.interner_mut().intern("Elf");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(elf);
    subtypes.0.insert(rogue);

    let self_filter = ObjectFilter {
        name: Some(name),
        ..ObjectFilter::default()
    };

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: self_filter,
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: on_combat_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                // GAP: "Whenever you attack" — no declare-attackers-step trigger;
                // approximated with "whenever a creature you control attacks".
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                },
                intervening_if: None,
                effect: maybe_unblockable,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_creature()],
            }),
    )
}

fn on_combat_damage(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(p) = trig.damaged_player() else {
        return Vec::new();
    };
    vec![
        Effect::LoseLife {
            player: p,
            amount: 1,
        },
        Effect::Mill {
            player: p,
            count: 1,
        },
        Effect::GainLife {
            player: trig.controller,
            amount: 1,
        },
        Effect::Scry {
            player: trig.controller,
            count: 1,
        },
    ]
}

fn maybe_unblockable(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{2}").expect("valid cost")),
        then: Box::new(Effect::CantBeBlocked {
            target: *id,
            duration: Duration::EndOfTurn,
        }),
        else_effect: None,
    }]
}
