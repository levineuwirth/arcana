//! Olivia, Opulent Outlaw — `{1}{R}{W}{B}` 3/3 Legendary Vampire Assassin.
//! Flying, lifelink.
//! "Whenever one or more outlaws you control deal combat damage to a player,
//! create a Treasure token."
//! "{3}, Sacrifice two Treasures: Put two +1/+1 counters on each creature you
//! control. Activate only as a sorcery."

use arcana_core::effects::{CommodityToken, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Olivia, Opulent Outlaw");
    let vampire = reg.interner_mut().intern("Vampire");
    let assassin = reg.interner_mut().intern("Assassin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(assassin);

    // Outlaw subtypes: Assassin, Mercenary, Pirate, Rogue, Warlock.
    let s_assassin = reg.interner_mut().intern("Assassin");
    let s_mercenary = reg.interner_mut().intern("Mercenary");
    let s_pirate = reg.interner_mut().intern("Pirate");
    let s_rogue = reg.interner_mut().intern("Rogue");
    let s_warlock = reg.interner_mut().intern("Warlock");
    let outlaw_source = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_subtypes_any(vec![s_assassin, s_mercenary, s_pirate, s_rogue, s_warlock]);

    // Treasure-to-sacrifice cost filter.
    let treasure_filter = script::subtype_filter(reg, "Treasure");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{W}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![
            arcana_core::effects::KeywordAbility::Flying,
            arcana_core::effects::KeywordAbility::Lifelink,
        ],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: "one or more outlaws ... deal combat damage" — the engine
                // fires per damage event, so multiple simultaneous attackers may
                // mint multiple Treasures rather than the printed single token.
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: outlaw_source,
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: make_treasure,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}, Sacrifice two Treasures: Put two +1/+1 counters on each creature you control. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}").expect("valid cost"),
                    sacrifice_other: Some(treasure_filter),
                    sacrifice_other_count: 2,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: counters_on_each_creature,
            }),
    )
}

fn make_treasure(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::CreateCommodityToken {
        controller: trig.controller,
        kind: CommodityToken::Treasure,
        count: 1,
    }]
}

fn counters_on_each_creature(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "Put two +1/+1 counters on each creature you control."
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        ctx.controller,
    );
    let mut effects = Vec::new();
    for id in ids {
        effects.push(Effect::AddCounters {
            target: id,
            kind: CounterKind::PlusOnePlusOne,
            count: 2,
        });
    }
    effects
}
