//! Merry, Warden of Isengard — `{1}{G}{W}` 1/4 Legendary Halfling Advisor.
//! "Partner with Pippin … (ETB tutor). Whenever one or more artifacts you
//! control enter, create a 1/1 white Soldier token with lifelink. This
//! ability triggers only once each turn."

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
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
    let name = reg.interner_mut().intern("Merry, Warden of Isengard");
    let halfling = reg.interner_mut().intern("Halfling");
    let advisor = reg.interner_mut().intern("Advisor");
    let _soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(halfling);
    subtypes.0.insert(advisor);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        // GAP: "Partner with" / "Partner" keywords are not in the supported
        // KeywordAbility surface.
        ..Default::default()
    };

    // GAP: "Partner with Pippin" ETB — optional, targets a player, and tutors
    // a named card to THAT player's hand. The optional + target-player +
    // named-tutor combination isn't expressible with the demonstrated API.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::permanent()
                        .with_types(TypeLine::ARTIFACT.into())
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: make_soldier,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::OncePerTurn,
                target_requirements: Vec::new(),
            }),
    )
}

fn make_soldier(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let soldier = reg.interner().lookup("Soldier").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(soldier);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: soldier,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![KeywordAbility::Lifelink],
            abilities: vec![],
        },
    }]
}
