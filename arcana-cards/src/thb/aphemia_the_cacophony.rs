//! Aphemia, the Cacophony — `{1}{B}` 2/1 Legendary Enchantment
//! Creature — Harpy with Flying.
//!
//! "At the beginning of your end step, you may exile an enchantment
//! card from your graveyard. If you do, create a 2/2 black Zombie
//! creature token."
//!
//! The optional exile is expressed via `ChooseAnyNumberFromZone`
//! (min-0 pick over enchantment cards in your graveyard, exiled). The
//! "If you do, create a token" payoff is conditioned on the exile
//! actually happening, which the demonstrated API can't couple to a
//! player-chosen variable-count pick — see GAP below.

use arcana_core::effects::{Effect, KeywordAbility, PickAction};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Aphemia, the Cacophony");
    let harpy = reg.interner_mut().intern("Harpy");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(harpy);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: end_step_exile,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn end_step_exile(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Optional exile of an enchantment card from your graveyard. The
    // zone already scopes to the controller's graveyard.
    let filter = ObjectFilter::new().with_types(TypeLine::ENCHANTMENT.into());
    vec![Effect::ChooseAnyNumberFromZone {
        chooser: trig.controller,
        zone: Zone::Graveyard(trig.controller),
        filter,
        action: PickAction::Exile,
    }]
    // GAP: "If you do, create a 2/2 black Zombie token" — the conditional
    // token-on-exile payoff can't be coupled to the player-chosen
    // variable-count graveyard pick with the demonstrated API.
}
