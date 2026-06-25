//! Howlsquad Heavy — `{2}{R}` 2/3 Creature — Goblin Mercenary.
//! Start your engines!
//! Other Goblins you control have haste.
//! At the beginning of combat on your turn, create a 1/1 red Goblin creature
//! token. That token attacks this combat if able.
//! Max speed — {T}: Add {R} for each Goblin you control.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Howlsquad Heavy");
    let goblin = reg.interner_mut().intern("Goblin");
    let mercenary = reg.interner_mut().intern("Mercenary");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(mercenary);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: "Start your engines!" and "Max speed" are not part of the usable
        // KeywordAbility surface (the speed-counter mechanic is not modeled);
        // omitted.
        ..Default::default()
    };

    // GAP (static): "Other Goblins you control have haste" — a static keyword-
    // granting anthem, not a triggered/activated ability; omitted.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::Combat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: make_goblin,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                // GAP: the "Max speed" precondition (requires max speed / 4+
                // speed counters) is not an expressible activation condition;
                // the {T}: Add {R} for each Goblin payload is wired faithfully.
                text: "Max speed — {T}: Add {R} for each Goblin you control.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_red_per_goblin,
            }),
    )
}

fn make_goblin(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let goblin = reg.interner().lookup("Goblin").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    // GAP: "That token attacks this combat if able" — the must_attack continuous
    // effect needs the token's ObjectId as its target, but Effect::CreateToken
    // allocates that id engine-side and does not expose it to the script (no
    // create-token-with-rider continuation primitive). The bare 1/1 red Goblin
    // token is created.
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: goblin,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

fn add_red_per_goblin(state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let n = script::count_matching(
        state,
        &script::subtype_filter(reg, "Goblin").controlled_by(ControllerConstraint::You),
        ctx.controller,
    );
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Red, ctx.source); n as usize],
    }]
}
