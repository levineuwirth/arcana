//! Jadar, Ghoulcaller of Nephalia — `{1}{B}` 1/1 Legendary Human
//! Wizard. "At the beginning of your end step, if you control no
//! creatures with decayed, create a 2/2 black Zombie creature token
//! with decayed."
//!
//! GAPs:
//! * The "if you control no creatures with decayed" intervening-if
//!   clause is not modeled — `intervening_if: None` and the trigger
//!   fires every end step. (Decayed is not in the keyword catalog,
//!   so there is no way to filter on it either.)
//! * The created Zombie token lacks the printed `decayed` keyword
//!   (not in the recognized keyword set). The token is emitted as
//!   a plain 2/2 black Zombie creature token.

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
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jadar, Ghoulcaller of Nephalia");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    // Pre-intern the token subtype so the resolver can look it up.
    let _zombie = reg.interner_mut().intern("Zombie");

    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                // GAP: intervening_if "if you control no creatures
                // with decayed" not modeled (no way to filter on
                // the decayed keyword).
                intervening_if: None,
                effect: create_zombie_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn create_zombie_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let zombie = reg
        .interner()
        .lookup("Zombie")
        .expect("Zombie interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    // GAP: printed "decayed" keyword on the token is not in the
    // recognized keyword catalog — token emitted without it.
    let token = TokenDefinition {
        name: zombie,
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken {
        controller: trig.controller,
        token,
    }]
}
