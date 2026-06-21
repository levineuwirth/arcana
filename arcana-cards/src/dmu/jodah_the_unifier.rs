//! Jodah, the Unifier — `{W}{U}{B}{R}{G}` 5/5 Legendary Human Wizard.
//! "Legendary creatures you control get +X/+X, where X is the number of
//!  legendary creatures you control." (dynamic anthem static — GAP)
//! "Whenever you cast a legendary spell from your hand, exile cards from the top
//!  of your library until you exile a legendary nonland card with lesser mana
//!  value. You may cast that card without paying its mana cost. Put the rest on
//!  the bottom of your library in a random order."
//!
//! The dynamic anthem is a static continuous effect with no expressible
//! primitive (GAP). The cast trigger fires on casting a legendary spell, but its
//! "exile until a legendary nonland card with lesser MV, cast it free" payload
//! has no matching primitive (Discover digs to a fixed MV with no legendary or
//! "from-hand"/relative-MV constraints), so the effect is GAP'd.

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Jodah, the Unifier");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    // GAP: "Legendary creatures you control get +X/+X, where X is the number of
    // legendary creatures you control." — a dynamic anthem static with no
    // expressible primitive.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}{B}{R}{G}").expect("valid cost")),
        colors: ColorSet::white()
            | ColorSet::blue()
            | ColorSet::black()
            | ColorSet::red()
            | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(
                        ObjectFilter::new().with_supertypes(
                            SupertypeSet::new().with(SupertypeSet::LEGENDARY),
                        ),
                    ),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: on_legendary_cast,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_legendary_cast(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "exile from top until a legendary nonland card with lesser mana value;
    // you may cast it free; rest on bottom in random order." — no primitive
    // expresses the legendary + relative-MV cutoff with a free cast; Discover
    // digs to a fixed MV without these constraints.
    Vec::new()
}
