//! Tellah, Great Sage — `{3}{U}{R}` 3/3 legendary blue-red Human Wizard.
//! "Whenever you cast a noncreature spell, create a 1/1 colorless Hero
//! creature token. If four or more mana was spent to cast that spell, draw
//! two cards. If eight or more mana was spent to cast that spell, sacrifice
//! Tellah and it deals that much damage to each opponent."
//! GAP: "if four or more / eight or more mana was spent" — mana-spent
//! tracking is not accessible at trigger resolution. Implementing token
//! creation only; conditional draw/damage return Vec::new().

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

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tellah, Great Sage");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);
    let _hero = reg.interner_mut().intern("Hero");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
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
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter::new()
                        .without_types(TypeLine::CREATURE.into())),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: noncreature_cast_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn noncreature_cast_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let hero = reg.interner().lookup("Hero").expect("Hero interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(hero);
    let token = TokenDefinition {
        name: hero,
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: "if 4+ mana spent draw 2; if 8+ mana spent sacrifice+damage" —
    // mana-spent amount not accessible at trigger resolution.
    vec![Effect::CreateToken { controller: trig.controller, token }]
}
