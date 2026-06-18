//! Sephiroth, Fallen Hero — `{3}{R}{W}` 7/5 Legendary Human Avatar Soldier.
//! Jenova Cells — Whenever Sephiroth attacks, you may put a cell counter on
//! target creature; (the "each modified creature has base 7/5" rider is a
//! continuous-effect GAP). The Reunion — {3}, Sacrifice a modified creature:
//! Return this card from your graveyard to the battlefield tapped.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sephiroth, Fallen Hero");
    let human = reg.interner_mut().intern("Human");
    let avatar = reg.interner_mut().intern("Avatar");
    let soldier = reg.interner_mut().intern("Soldier");
    let _cell = reg.interner_mut().intern("cell");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(avatar);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: jenova_cells,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_creature()],
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}, Sacrifice a modified creature: Return this card from your graveyard to the battlefield tapped.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}").expect("valid cost"),
                    // GAP: "Sacrifice a modified creature" — "modified" filter
                    // (Equipment/Auras/counters) not expressible; approximated
                    // as sacrificing any creature you control.
                    sacrifice_other: Some(ObjectFilter {
                        types: Some(TypeLine::CREATURE.into()),
                        ..ObjectFilter::default()
                    }),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Graveyard,
                is_instant_speed: false,
                face_gate: None,
                effect: the_reunion,
            }),
    )
}

fn jenova_cells(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    let cell = reg.interner().lookup("cell");
    let Some(cell) = cell else {
        return Vec::new();
    };
    // GAP: "Until end of turn, each modified creature you control has base
    // power and toughness 7/5" — board-wide continuous base-P/T set on a
    // dynamic ("modified") set not expressible here.
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::Named(cell),
        count: 1,
    }]
}

fn the_reunion(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "Return this card from your graveyard to the battlefield tapped" —
    // no self-reanimate-tapped effect in the demonstrated API.
    let _ = ctx.source;
    Vec::new()
}
