//! Unswerving Sloth — `{3}{W}{W}` 5/5 white Sloth Mount.
//! "Whenever this creature attacks while saddled, it gains indestructible
//!  until end of turn. Untap all creatures you control."
//! "Saddle 4" (the saddle keyword/cost is unsupported — GAP'd).
//!
//! Saddle is not in the supported KeywordAbility surface, and its activation
//! ("tap any number of other creatures with total power 4+; sorcery speed")
//! is not expressible, so it is GAP'd. The attack trigger likewise can't gate
//! on the "while saddled" state (no saddled condition), so it fires on every
//! attack — a documented over-fire GAP.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Unswerving Sloth");
    let sloth = reg.interner_mut().intern("Sloth");
    let mount = reg.interner_mut().intern("Mount");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sloth);
    subtypes.0.insert(mount);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        // GAP: Saddle is not in the supported KeywordAbility surface.
        keywords: vec![],
        ..Default::default()
    };

    // GAP: "Saddle 4" activated ability (tap any number of other creatures with
    // total power 4 or more; sorcery speed) is not expressible as an ActivationCost.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // GAP: cannot gate on the "while saddled" state — fires on every attack.
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: indestructible_and_untap,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn indestructible_and_untap(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    vec![Effect::Sequence(vec![
        Effect::GrantKeyword {
            target: trig.source,
            keyword: KeywordAbility::Indestructible,
            duration: Duration::EndOfTurn,
        },
        Effect::ForEach {
            targets: ids,
            effect: Box::new(Effect::Untap {
                target: arcana_core::objects::NULL_OBJECT_ID,
            }),
        },
    ])]
}
