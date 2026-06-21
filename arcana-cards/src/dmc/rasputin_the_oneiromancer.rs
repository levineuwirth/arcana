//! Rasputin, the Oneiromancer — `{1}{W}{U}` 4/1 Legendary Human Wizard.
//! ETB: a dream counter for each opponent + each opponent makes a Goblin.
//! Two tap activations spend dream counters; the variable-count mana
//! ability is GAP'd, and the Knight token's protection-from-red is GAP'd.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rasputin, the Oneiromancer");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let dream = reg.interner_mut().intern("dream");
    // Pre-intern token subtypes for the resolvers.
    let _goblin = reg.interner_mut().intern("Goblin");
    let _knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{U}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // "When Rasputin enters, put a dream counter on it for each
            // opponent you have. Each opponent creates a 1/1 red Goblin
            // creature token."
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_dreams_and_goblins,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // "{T}, Remove a dream counter from Rasputin: Create a 2/2
            // white Knight creature token with protection from red."
            // (Protection from red is GAP'd — the keyword surface has no
            // Protection; a plain 2/2 white Knight is minted.)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}, Remove a dream counter from Rasputin: Create a 2/2 white Knight creature token with protection from red.".into(),
                cost: ActivationCost {
                    tap: true,
                    remove_self_counter: Some((CounterKind::Named(dream), 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_knight,
            }),
        // GAP: "{T}, Remove one or more dream counters from Rasputin: Add
        // that much {C}." — variable-count counter removal feeding a
        // variable mana amount is not expressible with the fixed-count
        // remove_self_counter cost.
    )
}

/// ETB: dream counter per opponent + each opponent makes a 1/1 red Goblin.
fn etb_dreams_and_goblins(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let opps = script::opponents(state, trig.controller);
    let n = opps.len() as u32;
    let dream = match reg.interner().lookup("dream") {
        Some(s) => s,
        None => return Vec::new(),
    };
    let mut effects = Vec::new();
    if n > 0 {
        effects.push(Effect::AddCounters {
            target: trig.source,
            kind: CounterKind::Named(dream),
            count: n,
        });
    }
    let goblin = reg.interner().lookup("Goblin").unwrap_or_default();
    for opp in opps {
        let mut subtypes = SubtypeSet::default();
        subtypes.0.insert(goblin);
        effects.push(Effect::CreateToken {
            controller: opp,
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
        });
    }
    effects
}

/// Create a 2/2 white Knight token. GAP: protection from red.
fn make_knight(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let knight = reg.interner().lookup("Knight").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(knight);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: knight,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
