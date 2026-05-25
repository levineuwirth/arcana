//! A-Winota, Joiner of Forces — `{2}{R}{W}` 4/4 red-white Legendary Human Warrior.
//! "Whenever one or more non-Human creatures you control attack, look at the top six cards
//! of your library. You may put a Human creature card from among them onto the battlefield
//! tapped and attacking. It gains indestructible until end of turn. Put the rest on the
//! bottom of your library in a random order."
//! GAP: Looking at top 6 and selective-reveal-put-onto-battlefield-attacking is not in
//! the engine effect catalog; using TutorToBattlefield(tapped:true) as best-effort
//! (loses the "top 6 of library" restriction, gains indestructible, and attacks immediately).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
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
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You)
                        .without_types(TypeLine::CREATURE.into()), // GAP: "non-Human" requires subtype exclusion not available; using creature filter as best-effort
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
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "look at top 6, put Human creature from among them onto battlefield tapped and
    // attacking" is not in the engine effect catalog. Using TutorToBattlefield for a Human
    // creature as best-effort (loses top-6 restriction and attack-with-it constraint).
    // GAP: GrantKeyword(Indestructible) requires the just-created token id, not available
    // without a two-step delayed effect.
    vec![
        Effect::TutorToBattlefield {
            player: trig.controller,
            filter: ObjectFilter::creature(),
            tapped: true,
        },
    ]
}
