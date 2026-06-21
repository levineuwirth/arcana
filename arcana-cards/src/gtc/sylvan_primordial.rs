//! Sylvan Primordial — `{5}{G}{G}` 6/8 Avatar.
//! "Reach"
//! "When this creature enters, for each opponent, destroy target noncreature
//!  permanent that player controls. For each permanent destroyed this way, search
//!  your library for a Forest card and put that card onto the battlefield tapped.
//!  Then shuffle."
//!
//! Reach is a keyword. The ETB is wired for the dominant single-opponent case:
//! destroy one targeted noncreature permanent an opponent controls, then fetch a
//! Forest onto the battlefield tapped. GAP: the "for each opponent" multi-target
//! fan-out (one destroy + one Forest per opponent) cannot be expressed — target
//! count is fixed at registration — so only one opponent's destroy/fetch is wired.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sylvan Primordial");
    let avatar = reg.interner_mut().intern("Avatar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(avatar);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(8)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: destroy_and_fetch,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::permanent()
                        .without_types(TypeLine::CREATURE.into())
                        .controlled_by(ControllerConstraint::Opponent),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
        }),
    )
}

fn destroy_and_fetch(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // "For each permanent destroyed this way, search your library for a Forest card
    // and put that card onto the battlefield tapped." Single-opponent case: one
    // destroy → one Forest fetch.
    vec![
        Effect::DestroyPermanent { target: *id },
        Effect::TutorToBattlefield {
            player: trig.controller,
            filter: script::subtype_filter(reg, "Forest"),
            tapped: true,
        },
    ]
}
