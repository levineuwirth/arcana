//! Oltec Matterweaver — `{2}{W}` 2/4 Human Artificer.
//!
//! "Whenever you cast a creature spell, choose one —
//!  • Create a 1/1 colorless Gnome artifact creature token.
//!  • Create a token that's a copy of target artifact token you control."
//!
//! The "choose one" modal mechanism is only available for SPELL abilities,
//! not triggered abilities, in the demonstrated API. We implement the
//! first mode (the unconditional Gnome token) and GAP the modal choice +
//! the copy-token mode (which also needs a chosen target the trigger
//! modal can't carry).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::effects::TokenDefinition;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Oltec Matterweaver");
    let human = reg.interner_mut().intern("Human");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(artificer);

    // Pre-intern the token subtype so the resolver can rebuild it.
    let _gnome = reg.interner_mut().intern("Gnome");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SpellCast {
                filter: Some(
                    arcana_core::targets::ObjectFilter::new()
                        .with_types(TypeLine::CREATURE.into()),
                ),
                caster: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: make_gnome,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

// GAP: modal "choose one" on a triggered ability is not expressible; we
// always apply the first mode. GAP: second mode "create a token that's a
// copy of target artifact token you control" (modal target on a trigger).
fn make_gnome(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let gnome = reg.interner().lookup("Gnome").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(gnome);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: gnome,
            colors: ColorSet::colorless(),
            types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
