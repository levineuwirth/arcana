//! Gale, Holy Conduit — `{3}{W}{U}` 4/5 Legendary Creature — Human Wizard.
//! "Whenever you cast an instant or sorcery spell, create a 1/1 white Pegasus creature token with flying."

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gale, Holy Conduit");
    let human_sub = reg.interner_mut().intern("Human");
    let wizard_sub = reg.interner_mut().intern("Wizard");
    let _pegasus = reg.interner_mut().intern("Pegasus");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(wizard_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{U}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                filter: Some(ObjectFilter {
                    types_any: Some(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY)),
                    ..Default::default()
                }),
                caster: ControllerConstraint::You,
            },
                intervening_if: None,
                effect: gale_holy_conduit_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn gale_holy_conduit_trigger(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let pegasus_tok = reg.interner().lookup("Pegasus").expect("Pegasus interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(pegasus_tok);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: pegasus_tok,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![KeywordAbility::Flying],
            abilities: vec![],
        },
    }]
}
