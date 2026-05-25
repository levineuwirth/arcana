//! Myrsmith — `{1}{W}` 2/1 Human Artificer. "Whenever you cast an
//! artifact spell, you may pay {1}. If you do, create a 1/1
//! colorless Myr artifact creature token."
//!
//! GAP: the "may pay {1}" optional resolution cost is not
//! expressible in the current effect catalog (no optional-cost
//! Effect variant). The trigger condition (artifact spell cast by
//! you) and the token creation are faithfully encoded; the engine
//! will treat the token creation as unconditional until optional
//! costs are modeled.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Myrsmith");
    let human = reg.interner_mut().intern("Human");
    let artificer = reg.interner_mut().intern("Artificer");
    // Pre-intern the token subtype so the resolver can look it up.
    let _myr = reg.interner_mut().intern("Myr");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(artificer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter {
                        types_any: Some(TypeLine::ARTIFACT.into()),
                        ..Default::default()
                    }),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: create_myr_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn create_myr_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let myr = reg
        .interner()
        .lookup("Myr")
        .expect("Myr interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(myr);
    let token = TokenDefinition {
        name: myr,
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: oracle reads "you may pay {1}. If you do, create ..." —
    // the optional resolution cost is not modeled, so the token is
    // created unconditionally.
    vec![Effect::CreateToken { controller: trig.controller, token }]
}
