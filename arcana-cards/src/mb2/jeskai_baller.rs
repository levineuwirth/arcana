//! Jeskai Baller — `{2}{W}` 2/3 Human Athlete.
//! When you cast this spell, create a 1/1 white Athlete creature token.
//! Rebound — GAP (keyword not in the usable surface; the cast-from-exile
//! recast is unmodeled).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jeskai Baller");
    let human = reg.interner_mut().intern("Human");
    let athlete = reg.interner_mut().intern("Athlete");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(athlete);

    let self_name = reg.interner().lookup("Jeskai Baller");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: keyword Rebound is not in the usable keyword surface.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter {
                        name: self_name,
                        ..ObjectFilter::default()
                    }),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: cast_make_athlete,
                trigger_zones: vec![Zone::Stack],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn cast_make_athlete(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let athlete = reg.interner().lookup("Athlete").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(athlete);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: athlete,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
