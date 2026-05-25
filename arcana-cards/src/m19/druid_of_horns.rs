//! Druid of Horns — `{3}{G}` 2/3 green Human Druid.
//! "Whenever you cast an Aura spell that targets this creature, create a 3/3
//! green Beast creature token."
//! GAP: SpellCast filter for "Aura spell that targets this specific creature"
//! not expressible; using SpellCast Enchantment you as best-effort.

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
    let name = reg.interner_mut().intern("Druid of Horns");
    let human = reg.interner_mut().intern("Human");
    let druid = reg.interner_mut().intern("Druid");
    let _beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(druid);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: "Aura spell that targets this creature" filter not
                // expressible; using SpellCast Enchantment you as best-effort.
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(
                        ObjectFilter::new().with_types(TypeLine::ENCHANTMENT.into()),
                    ),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: on_aura_cast,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_aura_cast(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let beast = reg.interner().lookup("Beast").expect("Beast interned during register()");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(beast);
    let token = TokenDefinition {
        name: beast,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: trig.controller, token }]
}
