//! Halo Forager — `{1}{U}{B}` 3/1 Faerie Rogue with Flying.
//!
//! Oracle:
//!  * Flying.
//!  * When this creature enters, you may pay {X}. When you do, you may cast
//!    target instant or sorcery card with mana value X from a graveyard without
//!    paying its mana cost. If that spell would be put into a graveyard, exile
//!    it instead.
//!
//! Flying is wired. The ETB is an {X}-cost gate that then free-casts a
//! graveyard instant/sorcery of mana value X with an exile-replacement rider —
//! none of which is expressible (no free-cast-from-graveyard Effect, no dynamic
//! {X} optional cost). The whole ETB is GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Halo Forager");
    let faerie = reg.interner_mut().intern("Faerie");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(faerie);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_free_cast,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_free_cast(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may pay {X}. When you do, you may cast target instant or sorcery
    // with mana value X from a graveyard without paying its mana cost; exile it
    // instead of graveyard" — dynamic {X} optional cost + free-cast-from-
    // graveyard + zone-change replacement, none expressible.
    Vec::new()
}
