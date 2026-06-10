//! A-Winota, Joiner of Forces — `{2}{R}{W}` 4/4 red-white Legendary Human Warrior.
//! "Whenever one or more non-Human creatures you control attack, look at the top six cards
//! of your library. You may put a Human creature card from among them onto the battlefield
//! tapped and attacking. It gains indestructible until end of turn. Put the rest on the
//! bottom of your library in a random order."
//! The put half is wired: the trigger fn peeks the top six cards of the library at
//! resolution and puts the first Human creature card found onto the battlefield tapped
//! and attacking (Effect::PutOntoBattlefieldTappedAttacking).
//! GAP: the "you may" choice is rendered as a deterministic first-match pick (always
//! taken if present); the rest are NOT bottomed in random order (they stay on top);
//! the indestructible grant is omitted (the put re-ids the card per CR 400.7, so the
//! new battlefield id isn't visible to card code).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A-Winota, Joiner of Forces");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CreatureAttacks {
                    // "non-Human creature you control" — Human exclusion via
                    // without_subtype_sym. (Previously this had
                    // .without_types(CREATURE) on a creature() filter, which is
                    // unsatisfiable — the trigger could never fire.)
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You)
                        .without_subtype_sym(human),
                },
                intervening_if: None,
                effect: on_nonhuman_attacks,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_nonhuman_attacks(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // Look at the top six cards; put the first Human creature card found
    // onto the battlefield tapped and attacking (deterministic pick — see
    // module-doc GAP for the "you may" / rest-to-bottom / indestructible
    // divergences).
    let you = trig.controller;
    if you >= state.num_players() {
        return Vec::new();
    }
    let mut filter = ObjectFilter::creature();
    if let Some(human) = reg.interner().lookup("Human") {
        filter = filter.with_subtype_sym(human);
    }
    let chosen = state
        .player(you)
        .library_top_to_bottom
        .iter()
        .take(6)
        .copied()
        .find(|&id| {
            state
                .objects
                .get(id)
                .is_some_and(|o| filter.matches(o, state, you))
        });
    match chosen {
        Some(id) => vec![Effect::PutOntoBattlefieldTappedAttacking {
            target: id,
            controller: you,
        }],
        None => Vec::new(),
    }
}
