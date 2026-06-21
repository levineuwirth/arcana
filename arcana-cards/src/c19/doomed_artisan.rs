//! Doomed Artisan — `{2}{W}` 1/1 Human Artificer.
//!
//! Oracle:
//! * Sculptures you control can't attack or block.
//! * At the beginning of your end step, create a colorless Sculpture
//!   artifact creature token with "This token's power and toughness are
//!   each equal to the number of Sculptures you control."
//!
//! GAP: "Sculptures you control can't attack or block" is a board-wide
//! static restriction with no triggered/activated form — omitted.
//! GAP: the token's "*/* equal to the number of Sculptures you control"
//! self-referential dynamic P/T cannot be expressed on a TokenDefinition
//! (P/T must be a fixed PtValue). The token is minted as a colorless 0/0
//! Sculpture artifact creature with no abilities — a documented fidelity
//! gap on the printed characteristic-defining ability.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Doomed Artisan");
    let human = reg.interner_mut().intern("Human");
    let artificer = reg.interner_mut().intern("Artificer");
    let _sculpture = reg.interner_mut().intern("Sculpture");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(artificer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // GAP: static "Sculptures you control can't attack or block" omitted.

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::End,
                whose: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: make_sculpture,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// "create a colorless Sculpture artifact creature token …"
fn make_sculpture(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let sculpture = reg.interner().lookup("Sculpture").expect("Sculpture interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sculpture);
    let token = TokenDefinition {
        name: sculpture,
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        // GAP: dynamic P/T (= number of Sculptures you control) not
        // expressible; minted as 0/0.
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken {
        controller: trig.controller,
        token,
    }]
}
