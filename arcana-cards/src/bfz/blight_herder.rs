//! Blight Herder — `{5}` 4/5 colorless Eldrazi Processor.
//! "When you cast this spell, you may put two cards your opponents own
//! from exile into their owners' graveyards. If you do, create three
//! 1/1 colorless Eldrazi Scion creature tokens. They have 'Sacrifice
//! this token: Add {C}.'"
//! GAP: "put cards from exile into graveyards" not in engine catalog;
//! "when you cast this spell" trigger uses SpellCast on self.
//! Three Scion tokens created (Scion activated ability deferred engine work).

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
    let name = reg.interner_mut().intern("Blight Herder");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let processor = reg.interner_mut().intern("Processor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);
    subtypes.0.insert(processor);
    let _scion = reg.interner_mut().intern("Scion");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter::new()),
                    caster: ControllerConstraint::You,
                },
                // GAP: trigger should fire only when THIS spell is cast;
                // using SpellCast as closest approximation
                intervening_if: None,
                effect: cast_create_scions,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn cast_create_scions(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let scion = reg.interner().lookup("Scion").expect("Scion interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(scion);
    let token = TokenDefinition {
        name: scion,
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: "put 2 opponent exile cards into graveyards" not in engine catalog
    vec![
        Effect::CreateToken { controller: trig.controller, token: token.clone() },
        Effect::CreateToken { controller: trig.controller, token: token.clone() },
        Effect::CreateToken { controller: trig.controller, token },
    ]
}
