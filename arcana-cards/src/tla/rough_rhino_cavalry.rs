//! Rough Rhino Cavalry — `{4}{R}` 5/5 Human Mercenary.
//! "Firebending 2 (Whenever this creature attacks, add {R}{R}. This mana lasts
//!   until end of combat.)
//!  Exhaust — {8}: Put two +1/+1 counters on this creature. It gains trample
//!   until end of turn. (Activate each exhaust ability only once.)"
//!
//! "Firebending" / "Exhaust" are not in the usable KeywordAbility surface, but
//! both decompose: Firebending → a SelfAttacks trigger adding {R}{R} (the
//! until-end-of-combat duration on the mana is a fidelity gap); Exhaust → an
//! {8} activated ability (the once-per-game restriction is a GAP).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, ManaColor, PtValue, SubtypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rough Rhino Cavalry");
    let human = reg.interner_mut().intern("Human");
    let mercenary = reg.interner_mut().intern("Mercenary");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(mercenary);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        // GAP: Firebending / Exhaust are not usable KeywordAbility variants.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: firebending,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Exhaust — {8}: Put two +1/+1 counters on this creature. It gains trample until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{8}").expect("valid cost"),
                    // GAP: "Activate each exhaust ability only once" (once per
                    // game) has no matching ActivationCost field.
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: exhaust_pump,
            }),
    )
}

fn firebending(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // Fidelity gap: the "lasts until end of combat" rider on this mana is not
    // modeled — the mana is added to the pool normally.
    vec![Effect::AddMana {
        player: trig.controller,
        mana: vec![ManaUnit::plain(ManaColor::Red, trig.source); 2],
    }]
}

fn exhaust_pump(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![
        Effect::AddCounters {
            target: ctx.source,
            kind: CounterKind::PlusOnePlusOne,
            count: 2,
        },
        Effect::GrantKeyword {
            target: ctx.source,
            keyword: KeywordAbility::Trample,
            duration: Duration::EndOfTurn,
        },
    ]
}
