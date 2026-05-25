//! Dragonmaster Outcast — `{R}` 1/1 red Human Shaman.
//! "At the beginning of your upkeep, if you control six or more lands,
//! create a 5/5 red Dragon creature token with flying."
//! GAP: Intervening-if "if you control six or more lands" not expressible
//! as an engine condition; using intervening_if: None as best-effort.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dragonmaster Outcast");
    let human = reg.interner_mut().intern("Human");
    let shaman = reg.interner_mut().intern("Shaman");
    let _dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(shaman);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                // GAP: "if you control six or more lands" intervening-if not expressible.
                intervening_if: None,
                effect: on_upkeep,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_upkeep(state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let land_count = script::count_matching(
        state,
        &ObjectFilter::new()
            .with_types(TypeLine::LAND.into())
            .controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    if land_count < 6 {
        return Vec::new();
    }
    let dragon = reg.interner().lookup("Dragon").expect("Dragon interned during register()");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(dragon);
    let token = TokenDefinition {
        name: dragon,
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: trig.controller, token }]
}
