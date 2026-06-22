//! Zodiark, Umbral God — `{B}{B}{B}{B}{B}` 5/5 Legendary Creature — God.
//! Indestructible.
//! When Zodiark enters, each player sacrifices half the non-God creatures
//! they control of their choice, rounded down.
//! Whenever a player sacrifices another creature, put a +1/+1 counter on
//! Zodiark.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Zodiark, Umbral God");
    let god = reg.interner_mut().intern("God");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(god);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{B}{B}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Indestructible],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_each_player_sacs_half,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                // "Whenever a player sacrifices another creature, put a
                // +1/+1 counter on Zodiark."
                trigger_condition: TriggerCondition::Sacrificed {
                    filter: ObjectFilter::creature(),
                },
                intervening_if: None,
                effect: add_counter_on_sac,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_each_player_sacs_half(
    state: &GameState,
    _trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // Each player sacrifices half (rounded down) the NON-GOD creatures they
    // control. We compute floor(half) per player faithfully via a count
    // helper, then post one Sacrifice per player.
    let god = reg.interner().lookup("God");
    let mut out = Vec::new();
    for p in script::all_players(state) {
        let mut filter = ObjectFilter::creature().controlled_by(ControllerConstraint::You);
        if let Some(g) = god {
            filter = filter.without_subtype_sym(g);
        }
        let total = script::count_matching(state, &filter, p);
        let half = total / 2;
        // floor(half) is computed faithfully here; Effect::Sacrifice posts a
        // PickCards choice so player p selects WHICH non-God creatures to
        // sacrifice ("of their choice"). Both halves of the oracle are honored.
        if half > 0 {
            out.push(Effect::Sacrifice {
                player: p,
                filter: filter.clone(),
                count: half,
            });
        }
    }
    out
}

fn add_counter_on_sac(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
