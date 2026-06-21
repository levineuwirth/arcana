//! Feather, the Redeemed — `{R}{W}{W}` 3/4 Legendary Creature — Angel.
//!
//! Oracle:
//! * "Flying" — keyword.
//! * "Whenever you cast an instant or sorcery spell that targets a
//!   creature you control, exile that card instead of putting it into
//!   your graveyard as it resolves. If you do, return it to your hand at
//!   the beginning of the next end step." — a cast trigger that installs
//!   a resolution-replacement on the cast spell (exile-instead-of-
//!   graveyard) plus a delayed return. GAP: there is no demonstrated
//!   primitive that replaces a spell's resolution destination or that
//!   moves the just-cast card from the stack to exile and back; the
//!   "targets a creature you control" gate is also not expressible on a
//!   SpellCast filter. Trigger structure is emitted with an empty body.

use arcana_core::effects::Effect;
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
use arcana_core::effects::KeywordAbility;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Feather, the Redeemed");
    let angel = reg.interner_mut().intern("Angel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{W}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(
                        ObjectFilter::new()
                            .with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY)),
                    ),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: exile_and_return,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn exile_and_return(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "exile that card instead of putting it into your graveyard as
    // it resolves; return it to your hand at the next end step" — spell
    // resolution-destination replacement is not expressible.
    Vec::new()
}
