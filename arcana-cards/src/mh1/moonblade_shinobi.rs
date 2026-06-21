//! Moonblade Shinobi — `{3}{U}` 3/2 blue Human Ninja.
//! "Ninjutsu {2}{U}" — GAP: Ninjutsu is not an available KeywordAbility
//! variant (the return-an-unblocked-attacker / put-from-hand cast
//! mechanic isn't modeled), so it is omitted with `keywords: vec![]`.
//! "Whenever this creature deals combat damage to a player, create a
//! 1/1 blue Illusion creature token with flying." — wired as a
//! combat-damage-to-player trigger.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Moonblade Shinobi");
    let human = reg.interner_mut().intern("Human");
    let ninja = reg.interner_mut().intern("Ninja");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(ninja);
    let _ = reg.interner_mut().intern("Illusion");
    let self_name = reg.interner().lookup("Moonblade Shinobi");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![], // GAP: Ninjutsu {2}{U} unmodeled
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // "this creature deals combat damage" — no self-only damage
            // trigger variant exists; narrow the source filter to this
            // card's name controlled by you (closest faithful match).
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter {
                    name: self_name,
                    ..ObjectFilter::new().controlled_by(ControllerConstraint::You)
                },
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: make_illusion,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn make_illusion(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let illusion = reg.interner().lookup("Illusion").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(illusion);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: illusion,
            colors: ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![KeywordAbility::Flying],
            abilities: vec![],
        },
    }]
}
