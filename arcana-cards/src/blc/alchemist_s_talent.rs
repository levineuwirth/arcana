//! Alchemist's Talent — `{3}{R}` red Enchantment — Class.
//! Level 1 (base): When this Class enters, create two tapped Treasure tokens.
//! Level 2 ({1}{R}): Treasures you control have "{T}, Sacrifice this artifact:
//!   Add two mana of any one color."
//! Level 3 ({4}{R}): Whenever you cast a spell, if mana from a Treasure was spent
//!   to cast it, this Class deals damage equal to that spell's mana value to each
//!   opponent.
//!
//! GAP: Level 1 "tapped Treasure tokens" — CreateCommodityToken does not support
//! entering tapped; tokens enter untapped (fidelity gap).
//! GAP: Level 2 "Treasures you control have '{T}, Sac: Add two mana of any one color'"
//! — granting an activated ability to tokens is a continuous-effect engine debt; not
//! expressible. The Level 2 activation adds the Level counter only.
//! GAP: Level 3 "if mana from a Treasure was spent" — tracking which mana source paid
//! for a spell is not accessible via the script API. The triggered ability fires on
//! every spell cast (no Treasure-mana gate). "Damage equal to that spell's mana value"
//! is also not accessible. Both conditions are GAP'd; the trigger body returns Vec::new().

use arcana_core::effects::{CommodityToken, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry, EntersWithSpec,
};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Alchemist's Talent");
    let class_sub = reg.interner_mut().intern("Class");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(class_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Level,
                count: 1,
            })
            // Level 1 ETB: create two (tapped) Treasure tokens.
            // GAP: CreateCommodityToken doesn't enter tapped; fidelity gap.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_create_treasures,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Level 2: {1}{R} — requires level 1.
            // GAP: Treasure ability grant not expressible; only Level counter added.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{R}: Level 2.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{R}").unwrap(),
                    min_self_counters: Some((CounterKind::Level, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: level_up_to_2,
            })
            // Level 3: {4}{R} — requires level 2.
            // GAP: Treasure-mana-spent gate and mana-value-damage not expressible.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{4}{R}: Level 3.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}{R}").unwrap(),
                    min_self_counters: Some((CounterKind::Level, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: level_up_to_3,
            })
            // Level 3 triggered ability: whenever you cast a spell, if Treasure mana was spent,
            // deal damage equal to MV to each opponent.
            // GAP: no gate for Treasure-mana; no access to triggering spell's MV.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: level_3_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_create_treasures(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: tokens should enter tapped; CreateCommodityToken enters untapped.
    vec![
        Effect::CreateCommodityToken {
            controller: trig.controller,
            kind: CommodityToken::Treasure,
            count: 2,
        },
    ]
}

fn level_up_to_2(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Treasures you control have '{T}, Sacrifice this artifact: Add two mana
    // of any one color'" — ability-granting continuous effect deferred.
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::Level,
        count: 1,
    }]
}

fn level_up_to_3(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::Level,
        count: 1,
    }]
}

fn level_3_trigger(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "if mana from a Treasure was spent to cast it" — Treasure-mana tracking
    // not accessible via the script API.
    // GAP: "deal damage equal to that spell's mana value" — triggering spell's MV
    // not accessible via the script API.
    Vec::new()
}
