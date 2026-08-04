//! The Beast, Deathless Prince — `{2}{B}{R}` 6/6 Legendary Creature — Demon.
//! When you cast this spell, gain control of target creature until end of turn,
//! untap it, it gains menace and haste (GAP: no "when you cast this spell"
//! trigger condition).
//! The Beast enters tapped with six stun counters on it.
//! Whenever a creature deals combat damage to its owner, untap The Beast and
//! draw a card.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::TargetFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::targets::ObjectFilter;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Beast, Deathless Prince");
    let demon = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);

    // GAP: "When you cast this spell, gain control of target creature until end
    // of turn. Untap it. It gains menace and haste until end of turn." — there
    // is no "when you cast this spell" (cast-self) trigger condition; SpellCast
    // fires on OTHER spells, so it can't faithfully model this clause.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // "The Beast enters tapped with six stun counters on it." Modeled as
            // an ETB trigger that taps it and places the six stun counters.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_tapped_with_stun,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // "Whenever a creature deals combat damage to its owner, untap The
            // Beast and draw a card." GAP: "its owner" restriction is not
            // expressible — fires on combat damage dealt to any player.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::creature(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: untap_self_and_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_tapped_with_stun(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Sequence(vec![
        Effect::Tap { target: trig.source },
        Effect::AddCounters {
            target: trig.source,
            kind: CounterKind::Stun,
            count: 6,
        },
    ])]
}

fn untap_self_and_draw(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::Untap { target: trig.source },
        Effect::DrawCards { player: trig.controller, count: 1 },
    ]
}
