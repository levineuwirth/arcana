//! Mysterio, Master of Illusion — `{3}{U}` 3/3 blue Legendary Human Villain.
//! "When Mysterio enters, create a 3/3 blue Illusion Villain creature token for each
//! nontoken Villain you control. Exile those tokens when Mysterio leaves the battlefield."
//!
//! GAP: "for each nontoken Villain" requires dynamic count; the "exile those tokens when
//! Mysterio leaves" linked-trigger is also not expressible. Creating a single Illusion
//! token; the leave-battlefield exile is a GAP.

use arcana_core::effects::{Effect, TokenDefinition};
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
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mysterio, Master of Illusion");
    let human = reg.interner_mut().intern("Human");
    let villain = reg.interner_mut().intern("Villain");
    let illusion = reg.interner_mut().intern("Illusion");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(villain);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: on_etb,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_etb(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let illusion = reg.interner().lookup("Illusion").expect("Illusion interned during register()");
    let villain = reg.interner().lookup("Villain").expect("Villain interned during register()");
    let n = script::count_matching(
        state,
        &script::subtype_filter(reg, "Villain")
            .controlled_by(ControllerConstraint::You)
            .nontoken(),
        trig.controller,
    );
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(illusion);
    token_subtypes.0.insert(villain);
    let token = TokenDefinition {
        name: illusion,
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        abilities: vec![],
    };
    (0..n).map(|_| Effect::CreateToken { controller: trig.controller, token: token.clone() }).collect()
}
