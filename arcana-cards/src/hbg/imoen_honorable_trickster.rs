//! Imoen, Honorable Trickster — `{1}{W}{U}` 3/2 Legendary Human Rogue Wizard.
//!
//! * "Imoen, Honorable Trickster can't be blocked." — modeled as an ETB
//!   trigger applying a permanent can't-be-blocked to self (the engine has
//!   no static-ability hook in this surface; the WhileSourceOnBattlefield
//!   duration makes it effectively static once it enters).
//! * "Whenever Imoen deals combat damage to a player, you may exile an
//!   instant or sorcery card from your graveyard. If you do, put a +1/+1
//!   counter on each creature you control." — GAP: the "you may exile a
//!   card from your graveyard" optional cost gating the payoff is not
//!   expressible (no exile-from-graveyard OptionalPaymentKind).

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Imoen, Honorable Trickster");
    let human = reg.interner_mut().intern("Human");
    let rogue = reg.interner_mut().intern("Rogue");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(rogue);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{U}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: make_unblockable,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
        // GAP: "Whenever Imoen deals combat damage to a player, you may
        // exile an instant or sorcery card from your graveyard. If you do,
        // put a +1/+1 counter on each creature you control." — the
        // exile-from-graveyard optional cost gating the payoff has no
        // expressible OptionalPaymentKind.
    )
}

fn make_unblockable(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::CantBeBlocked {
        target: trig.source,
        duration: Duration::WhileSourceOnBattlefield,
    }]
}
