//! Dark Leo & Shredder — `{W}{B}` 1/3 legendary Mutant Ninja Turtle
//! Human.
//!
//! Rules text:
//! * Sneak {W}{B} — not a usable keyword variant; GAP'd.
//! * "Attacking Ninjas you control have deathtouch." — a static keyword
//!   grant to other permanents; GAP'd (no keyword-granting static).
//! * "Whenever Dark Leo & Shredder deal combat damage to a player,
//!   create a 1/1 black Ninja creature token. Then if you control five
//!   or more Ninjas, that player loses half their life, rounded up." —
//!   the token creation and the conditional half-life loss are both
//!   expressible (the conditional is evaluated at resolution).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dark Leo & Shredder");
    let mutant = reg.interner_mut().intern("Mutant");
    let ninja = reg.interner_mut().intern("Ninja");
    let turtle = reg.interner_mut().intern("Turtle");
    let human = reg.interner_mut().intern("Human");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(mutant);
    subtypes.0.insert(ninja);
    subtypes.0.insert(turtle);
    subtypes.0.insert(human);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: Sneak {W}{B} is not a usable keyword variant.
        keywords: vec![],
        ..Default::default()
    };

    // GAP: static "Attacking Ninjas you control have deathtouch."
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::new(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: combat_damage_ninja,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn combat_damage_ninja(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let ninja_name = reg.interner().lookup("Ninja").unwrap_or_default();
    let mut ninja_subtypes = SubtypeSet::default();
    if let Some(n) = reg.interner().lookup("Ninja") {
        ninja_subtypes.0.insert(n);
    }
    let mut effects = vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: ninja_name,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: ninja_subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }];

    // "Then if you control five or more Ninjas, that player loses half
    // their life, rounded up." Evaluated at resolution.
    let ninja_count = script::count_matching(
        state,
        &script::subtype_filter(reg, "Ninja").controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    if ninja_count >= 5 {
        if let Some(p) = trig.damaged_player() {
            let life = script::life(state, p).max(0);
            let half = ((life + 1) / 2) as u32;
            effects.push(Effect::LoseLife {
                player: p,
                amount: half,
            });
        }
    }

    effects
}
