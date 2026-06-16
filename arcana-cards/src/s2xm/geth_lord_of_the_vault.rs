//! Geth, Lord of the Vault — `{4}{B}{B}` 5/5 Legendary Phyrexian Zombie.
//! Intimidate.
//! "{X}{B}: Put target artifact or creature card with mana value X from an
//!  opponent's graveyard onto the battlefield under your control tapped.
//!  Then that player mills X cards."
//!
//! Intimidate is a base keyword. Scryfall's "Mill" tag is reminder text from
//! the ability, not an evergreen keyword. The activated ability's effect is
//! GAP'd: it needs a graveyard reanimation under YOUR control, tapped, gated
//! on mana-value == X, plus a follow-up mill of X — ReturnFromGraveyardToBattlefield
//! returns under the owner's control and cannot tap or gate on X.

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Geth, Lord of the Vault");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let zombie = reg.interner_mut().intern("Zombie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(zombie);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Intimidate],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{X}{B}: Put target artifact or creature card with mana value X from an opponent's graveyard onto the battlefield under your control tapped. Then that player mills X cards.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{X}{B}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: ObjectFilter::new()
                        .with_types_any(TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE))
                        .controlled_by(ControllerConstraint::Opponent),
                },
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: reanimate_and_mill,
        }),
    )
}

fn reanimate_and_mill(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: put the target onto the battlefield under YOUR control tapped (gated
    // on mana value == X) then mill that player X — reanimation returns under
    // the owner's control and cannot be tapped, X-gated, or chained to a mill.
    Vec::new()
}
